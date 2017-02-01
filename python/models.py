#!/usr/bin/env python
# -*- encoding: utf8 -*-

from sqlalchemy import Column, Integer, String, Date, Sequence, Boolean, ForeignKey
from sqlalchemy.orm import relationship
from sqlalchemy.ext.declarative import declarative_base
Base = declarative_base()


# Table Auteur
class Auteur(Base):
    __tablename__ = "auteurs"

    id = Column(Integer,Sequence('auteur_id_seq'),primary_key=True)
    prenom = Column(String(128))
    nom = Column(String(128))
    date_naissance = Column(Date,nullable=True)
    date_deces = Column(Date,nullable=True)
    nationalite = Column(String(128),nullable=True,default=None)
    livres = relationship('Livre',backref="auteurs")

    def __repr__(self):
        return "<Auteur(nom='%s',prenom='%s')>" %(self.prenom,self.nom)


# Table Editeur
class Editeur(Base):
    __tablename__ = "editeurs"

    id = Column(Integer,Sequence('editeur_id_seq'),primary_key=True)
    nom = Column(String(256))
    livres = relationship('Livre',backref="editeurs")

    def __repr__(self):
        return "<Editeur(nom = '%s')>" % (self.nom)


# Table Série
class Serie(Base):
    __tablename__ = "series"

    id = Column(Integer,Sequence('serie_id_seq'),primary_key=True)
    nom = Column(String(256))
    livres = relationship('Livre',backref="series")

    def __repr__(self):
        return "<Serie(nom = '%s' >" % (self.nom)


# Table Livre
class Livre(Base):
    __tablename__ = "livres"

    id = Column(Integer,Sequence('livre_id_seq'),primary_key=True)
    titre = Column(String(512))
    resume = Column(String(1024),nullable=True,default=None)
    couverture = Column(String(2048),nullable=True)
    auteur_id = Column(Integer,ForeignKey('auteurs.id'))
    editeur_id = Column(Integer,ForeignKey('editeurs.id'))
    serie_id = Column(Integer,ForeignKey('series.id'),nullable=True,default=None)
    publication = Column(Date,nullable=True,default=None)
    format = Column(String(32),nullable=True,default=None)
    pages = Column(Integer,nullable=True,default='0')
    ISBN = Column(String(32),nullable=True,default=None)
    read = Column(Boolean,default=False)

    def __repr__(self):
        return "<Livre(titre = '%s',isbn = '%s'>" % (self.titre,self.ISBN)

