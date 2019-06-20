#-------------------------------------------------
#
# Project created by QtCreator 2019-05-23T09:37:52
#
#-------------------------------------------------

QT       += core gui

greaterThan(QT_MAJOR_VERSION, 4): QT += widgets

TARGET = zecpaperwalletui
TEMPLATE = app

MOC_DIR = bin
OBJECTS_DIR = bin
UI_DIR = src


# The following define makes your compiler emit warnings if you use
# any feature of Qt which has been marked as deprecated (the exact warnings
# depend on your compiler). Please consult the documentation of the
# deprecated API in order to know how to port your code away from it.
DEFINES += QT_DEPRECATED_WARNINGS

# You can also make your code fail to compile if you use deprecated APIs.
# In order to do so, uncomment the following line.
# You can also select to disable deprecated APIs only up to a certain version of Qt.
#DEFINES += QT_DISABLE_DEPRECATED_BEFORE=0x060000    # disables all the APIs deprecated before Qt 6.0.0

CONFIG += c++14

CONFIG += precompile_header

PRECOMPILED_HEADER = src/precompiled.h


SOURCES += \
        src/main.cpp \
        src/mainwindow.cpp \
        src/qrcodelabel.cpp \
        src/qrcode/BitBuffer.cpp \
        src/qrcode/QrCode.cpp \
        src/qrcode/QrSegment.cpp 

HEADERS += \
        src/mainwindow.h \
        src/qrcodelabel.h \
        src/precompiled.h \
        src/qrcode/BitBuffer.hpp \
        src/qrcode/QrCode.hpp \
        src/qrcode/QrSegment.hpp \
        qtlib/src/zecpaperrust.h

FORMS += \
        src/about.ui \
        src/mainwindow.ui \
        src/wallet.ui

# Rust library
INCLUDEPATH += $$PWD/qtlib/src
DEPENDPATH  += $$PWD/qtlib/src

unix:        librust.target   = $$PWD/qtlib/target/release/libzecpaperrust.a
else:win32:  librust.target   = $$PWD/qtlib/target/x86_64-pc-windows-gnu/release/zecpaperrust.lib

unix:        librust.commands = $(MAKE) -C $$PWD/qtlib 
else:win32:  librust.commands = $(MAKE) -C $$PWD/qtlib winrelease

librustclean.commands = "rm -rf $$PWD/qtlib/target"
distclean.depends += librustclean


QMAKE_EXTRA_TARGETS += librust librustclean distclean
QMAKE_CLEAN += $$PWD/qtlib/target/release/libzecpaperrust.a

# Default rules for deployment.
qnx: target.path = /tmp/$${TARGET}/bin
else: unix:!android: target.path = /opt/$${TARGET}/bin
!isEmpty(target.path): INSTALLS += target


win32: LIBS += -L$$PWD/qtlib/target/x86_64-pc-windows-gnu/release -lzecpaperrust
else:macx: LIBS += -L$$PWD/qtlib/target/release -lzecpaperrust -framework Security -framework Foundation
else:unix: LIBS += -L$$PWD/qtlib/target/release -lzecpaperrust -ldl

win32: PRE_TARGETDEPS += $$PWD/qtlib/target/x86_64-pc-windows-gnu/release/zecpaperrust.lib
else:unix::PRE_TARGETDEPS += $$PWD/qtlib/target/release/libzecpaperrust.a
