#!/usr/bin/env python
# -*- encoding: utf8 -*-

# Installe la base de données
from models import *

from sqlalchemy import create_engine
#dans un premier temps en mémoire
engine = create_engine('sqlite:///:memory:',echo=True)
Base.metadata.create_all(engine)

