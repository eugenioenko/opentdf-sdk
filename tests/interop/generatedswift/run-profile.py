#!/usr/bin/env python3
"""Run the shared native-importer EC/DPoP matrix with the Swift consumer."""
import os
import runpy
from pathlib import Path

os.environ['TDF_INTEROP_TARGET'] = 'swift'
runpy.run_path(str(Path(__file__).resolve().parents[1]/'generatedpython/run-profile.py'), run_name='__main__')
