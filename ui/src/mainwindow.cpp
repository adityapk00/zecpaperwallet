#include <cstring>

#include "precompiled.h"

#include "mainwindow.h"
#include "ui_mainwindow.h"
#include "ui_wallet.h"
#include "ui_about.h"

#include "zecpaperrust.h"

void SaveRestore(QDialog* d) {
    d->restoreGeometry(QSettings().value(d->objectName() % "geometry").toByteArray());

    QObject::connect(d, &QDialog::finished, [=](auto) {
        QSettings().setValue(d->objectName() % "geometry", d->saveGeometry());
    });
}


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

    // Setup fixed with fonts
    //const QFont fixedFont = QFontDatabase::systemFont(QFontDatabase::FixedFont);
    
    w.qrAddress->setQrcodeString(address);
    w.lblAddress->setText(SplitIntoLines(address, 39));
    //w.lblAddress->setFont(fixedFont);

    w.qrPrivateKey->setQrcodeString(pk);
    w.lblPrivateKey->setText(SplitIntoLines(pk, 59));
    //w.lblPrivateKey->setFont(fixedFont);
}

/**
 * Generate wallets and return a JSON.
 */
QString Generate(int zaddrs, int taddrs, QString entropy) {
    // Call into rust to get the addresses
    char* wallet = rust_generate_wallet(false, zaddrs, taddrs, entropy.toStdString().c_str());
    QString walletJson(wallet);
    
    // We'll overwrite the privatekeys for safety before sending it back to rust
    std::memset(wallet, 0, strlen(wallet)); 
    rust_free_string(wallet);

    return walletJson;
}

void MainWindow::populateWallets() {
    // First, get the numbers
    int zaddrs = ui->txtzaddrs->text().toInt();
    int taddrs = ui->txttaddrs->text().toInt();

    QString entropy = ui->txtEntropy->text();

    currentWallets = Generate(zaddrs, taddrs, entropy);

    // Then, clear the Scroll area
    auto children  = ui->scroll->findChildren<QGroupBox *>();
    for (int i=0; i < children.length(); i++) {
        delete children[i];
    }

    // Then add the new wallets
    auto json = QJsonDocument::fromJson(currentWallets.toUtf8());
    for (int i=0; i < json.array().size(); i++) {
        auto addr = json.array()[i].toObject()["address"].toString();
        auto pk   = json.array()[i].toObject()["private_key"].toString();

        AddWallet(addr, pk, ui->scroll);
    }    
}

void MainWindow::SaveAsPDFButton() {
    // If there's nothing to save, just exit
    if (currentWallets.isEmpty())
        return;

    // Get a save file name
    auto filename = QFileDialog::getSaveFileName(this, tr("Save as PDF"), "", tr("PDF Files (*.pdf)"));
    if (!filename.isEmpty()) {
        bool success = rust_save_as_pdf(this->currentWallets.toStdString().c_str(), filename.toStdString().c_str());
        if (success) {
            QMessageBox::information(this, tr("Saved!"), tr("The wallets were saved to ") + filename);
        } else {
            QMessageBox::warning(this, tr("Failed to save file"), 
                    tr("Couldn't save the file for some unknown reason"));
        }
    }
}

void MainWindow::SaveAsJSONButton() {
    // If there's nothing to save, just exit
    if (currentWallets.isEmpty())
        return;
        
    auto filename = QFileDialog::getSaveFileName(this, tr("Save as text"), "", tr("Text Files (*.txt)"));
    if (!filename.isEmpty()) {
        QFile file(filename);
        if (file.open(QIODevice::WriteOnly))
        {
            QTextStream stream(&file);
            stream << this->currentWallets << endl;
        }
    }
}

void MainWindow::closeEvent(QCloseEvent*) {
    QSettings().setValue("geometry", saveGeometry());
}

MainWindow::MainWindow(QWidget *parent) :
    QMainWindow(parent),
    ui(new Ui::MainWindow)
{
    ui->setupUi(this);   

    // Save the geometry
    restoreGeometry(QSettings().value("geometry").toByteArray());

    // First, set up the validators for the number fields
    intValidator = new QIntValidator(0, 25);
    ui->txttaddrs->setValidator(intValidator);
    ui->txtzaddrs->setValidator(intValidator);

    // Wire up the generate button
    QObject::connect(ui->btnGenerate, &QPushButton::clicked, [=]() {
        this->populateWallets();
    });

    // Close button
    QObject::connect(ui->actionExit, &QAction::triggered, this, &MainWindow::close);

    // Help site
    QObject::connect(ui->actionHelp_site, &QAction::triggered, [=]() {
        QDesktopServices::openUrl(QUrl("https://docs.zecwallet.co/paper")); 
    });

    // About button
    QObject::connect(ui->actionAbout, &QAction::triggered, [=]() {
        QDialog ad(this);
        Ui_AboutDialog d;
        d.setupUi(&ad);

        SaveRestore(&ad);

        ad.exec();
    });

    // Save as PDF 
    // Button
    QObject::connect(ui->btnSavePDF, &QPushButton::clicked, [=]() {
        SaveAsPDFButton();
    });
    // Menu item
    QObject::connect(ui->actionSave_as_PDF, &QAction::triggered, this, &MainWindow::SaveAsPDFButton);

    // Save as JSON
    QObject::connect(ui->btnSaveJSON, &QPushButton::clicked, [=]() {
        SaveAsJSONButton();
    });
    // Menu item
    QObject::connect(ui->actionSave_as_JSON, &QAction::triggered, this, &MainWindow::SaveAsJSONButton);

    // Generate the default wallets
    populateWallets();
    
}

MainWindow::~MainWindow()
{
    delete ui;
    delete intValidator;
}
