"""Offline source-to-fixture verification; no third-party Python packages."""
import argparse
from decimal import Decimal
import hashlib
from html.parser import HTMLParser
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / 'data/port-la-2023'
MONTHS = ['January', 'February', 'March', 'April', 'May', 'June', 'July',
          'August', 'September', 'October', 'November', 'December']
COLUMNS = ['loaded_imports', 'empty_imports', 'total_imports', 'loaded_exports',
           'empty_exports', 'total_exports', 'total_teu']
EXPECTED_HASHES = {
    'table.html.txt': '835226206a91cd8446ae0b0bf1f2aa55297b08b4e82c7b7f881256c69057ac0e',
    'news.html.txt': '9d95b79d08922615b1e630169d062105d5c8b1e02f0cd776803766f99731406e',
    'disclaimer.html.txt': '943548f4624e4caf40947dbe96b0e306e1005f7261d100da1e3c1febbd256cba',
}


class SourceTable(HTMLParser):
    def __init__(self):
        super().__init__()
        self.active = False
        self.cell = None
        self.row = []
        self.rows = []
        self.matches = 0

    def handle_starttag(self, tag, attrs):
        if tag == 'table' and dict(attrs).get('summary') == '2023 Container Statistics':
            self.active = True
            self.matches += 1
        if not self.active:
            return
        if tag == 'tr':
            self.row = []
        if tag in ('td', 'th'):
            self.cell = []

    def handle_data(self, text):
        if self.active and self.cell is not None:
            self.cell.append(text)

    def handle_endtag(self, tag):
        if not self.active:
            return
        if tag in ('td', 'th') and self.cell is not None:
            self.row.append(' '.join(''.join(self.cell).split()))
            self.cell = None
        if tag == 'tr':
            self.rows.append(self.row)
        if tag == 'table':
            self.active = False


class Text(HTMLParser):
    def __init__(self):
        super().__init__()
        self.parts = []

    def handle_data(self, text):
        self.parts.append(text)


def centi(value):
    number = Decimal(value.replace(',', '')) * 100
    if not number.is_finite() or number < 0 or number != number.to_integral_value():
        raise ValueError('Source quantity is not a nonnegative exact centi-TEU value')
    return int(number)


def extract():
    custody = json.loads((DATA / 'custody.json').read_text(encoding='utf-8'))
    if {f['file']: f['sha256'] for f in custody['files']} != EXPECTED_HASHES:
        raise ValueError('Custody manifest identity drift')
    for record in custody['files']:
        body = (DATA / record['file']).read_bytes()
        if len(body) != record['bytes'] or hashlib.sha256(body).hexdigest() != record['sha256']:
            raise ValueError('Publisher response bytes changed: ' + record['file'])
    parser = SourceTable()
    parser.feed((DATA / 'table.html.txt').read_text(encoding='utf-8'))
    if parser.matches != 1 or len(parser.rows) != 15:
        raise ValueError('Expected exactly one 15-row publisher table')
    header = ['', 'Loaded Imports', 'Empty Imports', 'Total Imports', 'Loaded Exports',
              'Empty Exports', 'Total Exports', 'Total TEUs', 'Prior Year Change']
    if parser.rows[0] != header or any(len(row) != 9 for row in parser.rows):
        raise ValueError('Publisher table column drift')
    months = []
    for index, row in enumerate(parser.rows[1:13]):
        if row[0] != MONTHS[index]:
            raise ValueError('Missing, duplicate or out-of-order month')
        months.append({'month': index + 1, 'name': row[0], 'centi_teu': [centi(c) for c in row[1:8]],
                       'reported_prior_year_change': row[8]})
    annual, fiscal = parser.rows[13:]
    if annual[0] != 'Total Calendar Year 2023' or fiscal[0] != 'Total Fiscal Year 2023/24 (in progress)':
        raise ValueError('Calendar/fiscal source window drift')
    news = Text()
    news.feed((DATA / 'news.html.txt').read_text(encoding='utf-8'))
    text = ' '.join(' '.join(news.parts).split())
    if 'Jan. 16, 2024' not in text:
        raise ValueError('News publication date missing')
    annual_news = re.search(r'finished 2023 handling ([0-9,]+) TEUs', text)
    december_news = re.search(r'processing ([0-9,]+) Twenty-Foot Equivalent Units', text)
    if not annual_news or not december_news:
        raise ValueError('News quantity locator drift')
    return {
        'schema': 'harbor.frozen-port-la-container-source.v1', 'year': 2023,
        'gateway': 'Port of Los Angeles', 'scale': 'local gateway source context',
        'unit': 'centi-TEU (100 units = 1 TEU)', 'columns': COLUMNS,
        'months': months, 'reported_calendar_year_centi_teu': [centi(c) for c in annual[1:8]],
        'reported_calendar_year_change': annual[8],
        'partial_fiscal_year': {'label': fiscal[0], 'included_in_calendar_sums': False,
                                'centi_teu': [centi(c) for c in fiscal[1:8]],
                                'reported_prior_year_change': fiscal[8]},
        'news': {'publication_date': '2024-01-16', 'precision': 'whole TEU as published',
                 'annual_centi_teu': centi(annual_news[1]), 'december_centi_teu': centi(december_news[1])},
        'custody': custody,
    }


def verify():
    expected = extract()
    actual = json.loads((DATA / 'source.json').read_text(encoding='utf-8'))
    if actual != expected:
        raise ValueError('Frozen fixture differs from original publisher responses')
    print('Port LA: 12 months, 7 columns, separate calendar/fiscal rows and 2 news quantities verified; 3 original payload hashes match')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--write', action='store_true', help='Materialize a reviewed source fixture')
    if parser.parse_args().write:
        (DATA / 'source.json').write_bytes((json.dumps(extract(), indent=2) + '\n').encode('utf-8'))
    verify()
