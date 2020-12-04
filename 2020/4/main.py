#!/usr/bin/env python3

def get_passport(input):
    passport = ""

    for x in input:
        if x == '\n':
            yield passport
            passport = ""
        passport += x
    
    yield passport

def is_valid(passport):
    required_fields = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"]

    if all(x in passport for x in required_fields):
        passport = passport.replace("\n", " ")
        for x in required_fields:
            field_data_start = passport.find(x) + len(x)
            field_data_end = passport[field_data_start:].find(" ")
            field_data = passport[field_data_start:field_data_start+field_data_end+1].strip()

            if x == "byr:":
                if len(field_data) > 4:
                    return False
                try:
                    data = int(field_data)
                except:
                    return False

                if data < 1920 or data > 2002:
                    return False 
                
            if x == "iyr:":
                if len(field_data) > 4:
                    return False
                data = int(field_data)

                if data < 2010 or data > 2020:
                    return False 
            if x == "eyr:":
                if len(field_data) > 4:
                    return False
                data = int(field_data)

                if data < 2020 or data > 2030:
                    return False 
            if x == "hgt:":
                if not "cm" in field_data and not "in" in field_data:
                    return False
                
                height = int(field_data[:len(field_data) - 2])

                if "cm" in field_data and (height < 150 or height > 193):
                    return False
                if "in" in field_data and (height < 59 or height > 76):
                    return False
            if x == "hcl:":
                valid = "0123456789abcdef"
                if field_data[0] != '#':
                    return False
                
                for z in field_data[1:]:
                    if z not in valid:
                        return False
                if len(field_data) != 7:
                    return False
                pass
            if x == "ecl:":
                valid = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                if not field_data in valid:
                    return False
            if x == "pid:":
                if len(field_data) != 9:
                    return False
                try:
                    data = int(field_data)
                except:
                    return False
        
        return True
    return False


def main():
    input = open("input.txt", 'r').readlines()

    valid = 0
    for x in get_passport(input):
        if (is_valid(x)):
            valid += 1
    
    print("Valid:", valid)



main()