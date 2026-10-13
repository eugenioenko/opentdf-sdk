#!/usr/bin/env python3
"""Offline controlled checks of the rust consumer's handle API, driven by the
shared fixture harness (tests/interop/delivery/controlled_platform.py)."""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'delivery'))
import controlled_platform

controlled_platform.main('rust', [str(Path(__file__).resolve().with_name('consumer.sh'))])
