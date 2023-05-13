# Bankcsv

Read a csv file you downloaded from your bank and parse out what's important.

## Summary

I downloaded a csv file from my bank that contains all the activities for my checking account and another file for my credit card. I want to read the file into a Vec of structs then write the "clean" data to a file.

Why? I can do some queries on my spending. Eventually I'll want to save the data to a database, like SQLite and do more focused queries use Rust and SQL.

So, I am starting over! I tried to incorporate too many things at one time, thus both confusing and overwhelming myself. 

Start wicked simple, get it working, then build from there. 

## Steps

### Step One

- Read the file.
  - Hardcode the file name and location. Later you can use clap or struct-opt to prompt for a file name.
- Don’t worry about creating modules or whatever they’re called. If you have to, just put everything into one file. Later you can split things apart.
- Read data into a struct
- Be able to query and display data from the data in memory

### Step Two
 
- The _field_ has stuff like "PURCHASE AUTHORIZED ON 01/09 HUNGRYSAM 123456789 HICKORY WI S9876543211234 CARD 12345" I want to add 'Hungry Sam' to the 'payee' field.

### Step Three
- Based on the 'payee' field I want to populate the category field.
- Not sure if I should create a table that I can cross reference in code or use a config file. The config file would make it easier to add or change values. Thinking ...