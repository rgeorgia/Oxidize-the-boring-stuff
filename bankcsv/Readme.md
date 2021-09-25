# Oxidize The Boring Stuff

## bankcsv

Read in the csv files from the data directory and create a clean csv file in the clean-data directory.

1. Read the file
2. Parse columns into AccountEvent struct
3. Write clean data to clean-data/file.csv

### Parsing Content (step 2)

#### Raw File Format

Checking.csv

Date, Money, "*", Check Numbers, Payee - plus other stuff

CreditCard.csv

Date, Money, "*", "", Payee - plus other stuff

#### Final Format

Date, Money, Check Number, Payee, Category, Account

### Data Dictionary


