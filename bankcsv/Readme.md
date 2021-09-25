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

|field|type|format|comment|
|-----|----|------|-------|
|Date|date|mm-dd-yyyy||
|Money|decimal|use decimal to prevent float rounding|
|Check Number|string|"dddd"|use string because we don't need to do math with check numbers|
|Payee|string||Maybe need to compile a list?|
|Category|Enum|Enum|Make a list of categories|
|Account|Enum|Enum|Two accounts, checking, credit cards |
