#include "precompiled.h"

#include "mainwindow.h"
#include "ui_mainwindow.h"
#include "ui_wallet.h"

#include "zecpaperrust.h"

QString SplitIntoLines(QString s, int maxlen) {
    if (s.length() <= maxlen)
        return s;

    QStringList ans;
    int start = 0;
    for (int i=0; i < (s.length() / maxlen) + 1; i++) {
        ans << s.mid(start, maxlen);
        start += maxlen;
    }

    return ans.join("\n");
}

/**
 * Add a wallet (address + pk) section to the given vertical layout
 */
void AddWallet(QString address, QString pk, QWidget* scroll) {
    Ui_WalletWidget w;
    auto g1 = new QGroupBox(scroll);
    w.setupUi(g1);
    scroll->layout()->addWidget(g1);

    w.qrAddress->setQrcodeString(address);
    w.lblAddress->setText(SplitIntoLines(address, 44));

    w.qrPrivateKey->setQrcodeString(pk);
    w.lblPrivateKey->setText(SplitIntoLines(pk, 59));
}

MainWindow::MainWindow(QWidget *parent) :
    QMainWindow(parent),
    ui(new Ui::MainWindow)
{
    ui->setupUi(this);

    // Setup fixed with fonts
    // const QFont fixedFont = QFontDatabase::systemFont(QFontDatabase::FixedFont);
    // ui->lblAddress->setFont(fixedFont);
    // ui->lblPrivateKey->setFont(fixedFont);

    // Call into rust to get the addresses
    char* wallet = rust_generate_wallet(true, 1, 1, "entropy");
    QString walletJson(wallet);
    rust_free_string(wallet);

    auto json = QJsonDocument::fromJson(walletJson.toUtf8());
    for (int i=0; i < json.array().size(); i++) {
        auto addr = json.array()[i].toObject()["address"].toString();
        auto pk   = json.array()[i].toObject()["private_key"].toString();

        AddWallet(addr, pk, ui->scroll);
    }
    
}



MainWindow::~MainWindow()
{
    delete ui;
}
