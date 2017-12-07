#!/usr/bin/python

import datetime
from os import path

def variant(num):
    if num == 1:
        return 'Fold::One'
    elif num == 2:
        return 'Fold::Two'
    elif num == 3:
        return 'Fold::Three'

def replacement(chars):
    chars = chars.split(' ')
    chars_len = len(chars)
    inside = ', '.join(["'\\u{%s}'" % c for c in chars])
    return '%s(%s,)' % (variant(chars_len), inside)


txt = open('./scripts/CaseFolding.txt')
rs = open(path.abspath('./src/unicode/map.rs'), 'w')

rs.write('// Generated by scripts/mapgen.py\n')
rs.write('// %s\n' % datetime.date.today())
rs.write('\n')
rs.write('use super::fold::Fold;\n\n')
rs.write("pub fn lookup(orig: char) -> Fold {\n")
rs.write('    match orig as u32 {\n')
for line in txt.readlines():
    if line[0] is not '#':
        parts = line.split('; ')
        if len(parts) > 2 and parts[1] in 'CF':
            rs.write("        0x%s => %s,\n" % (parts[0], replacement(parts[2])))
rs.write('        _ => Fold::One(orig,)\n')
rs.write('    }\n')
rs.write('}\n')


