#!/usr/bin/env python3
import sys
import os
import logging

# Configure logging to see console.log() output
logging.basicConfig(level=logging.INFO, format='%(message)s')

# Add temper-core and ormery to Python path
project_root = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(project_root, 'temper.out/py/temper-core'))
sys.path.insert(0, os.path.join(project_root, 'temper.out/py/ormery'))

# Import and run main
from ormery.ormery import main
main()
