#!/usr/bin/env python
# Copyright 2015 ARM Limited
#
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

# standard library modules, , ,
import unittest

# internal modules:
from yotta.test.cli import util
from yotta.test.cli import cli

Test_Outdated = {
'module.json':'''{
  "name": "test-outdated",
  "version": "0.0.0",
  "description": "Test yotta outdated",
  "author": "James Crosby <james.crosby@arm.com>",
  "license": "Apache-2.0",
  "dependencies":{
    "test-testing-dummy": "*"
  }
}''',
'source/foo.c':'''#include "stdio.h"
int foo(){
    printf("foo!\\n");
    return 7;
}''',
# test-testing-dummy v0.0.1 (a newer version is available from the registry,
# and will be installed by yt up)
'yotta_modules/test-testing-dummy/module.json':'''{
  "name": "test-testing-dummy",
  "version": "0.0.1",
  "description": "Test yotta's compilation of tests.",
  "author": "James Crosby <james.crosby@arm.com>",
  "license": "Apache-2.0"
}
'''
}

class TestCLIOutdated(unittest.TestCase):
    def test_outdated(self):
        path = util.writeTestFiles(Test_Outdated, True)

        stdout, stderr, statuscode = cli.run(['-t', 'x86-linux-native', 'outdated'], cwd=path)
        self.assertNotEqual(statuscode, 0)
        self.assertIn('test-testing-dummy', stdout + stderr)

        util.rmRf(path)

    def test_notOutdated(self):
        path = util.writeTestFiles(Test_Outdated, True)

        stdout, stderr, statuscode = cli.run(['-t', 'x86-linux-native', 'up'], cwd=path)
        self.assertEqual(statuscode, 0)

        stdout, stderr, statuscode = cli.run(['-t', 'x86-linux-native', 'outdated'], cwd=path)
        self.assertEqual(statuscode, 0)
        self.assertNotIn('test-testing-dummy', stdout + stderr)

        util.rmRf(path)
