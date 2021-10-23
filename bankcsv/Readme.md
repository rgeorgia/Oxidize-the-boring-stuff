# Bankcsv

Read a csv file you downloaded from your bank and parse out what's important.

## Summary

I download a csv file from my bank that contains all activity for my checking account and another file for my credit card. I want to read in the file into a vec of structs then write the "clean" data to a file.

Why? So I can do some queries on my spending. Eventually I'll want to save the data to a database, like sqlite and do more focused queries using Rust and SQL.

## Steps

### Step One

- Read the file.
- Read data into a struct
- Be able to query and display data from the data in memory

### Step Two

- Clean up the description field. 
- That _field_ has stuff like "PURCHASE AUTHORIZED ON 01/09 HUNGRYSAM 123456789 HICKORY NC S9876543211234 CARD 6666"
- All I want is HUNGRYSAM and I want to change it to Hungry Sam.
- Add another a category to the activity struct.
- Maybe an enum Category