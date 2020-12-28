use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file.");
    let entries: Vec<&str> = contents.split("\n\n").collect();

    let num_valid = count_valid(&entries);
    println!("Part 1: {}.", num_valid);
    let num_valid2 = count_valid2(&entries);
    println!("Part 2: {}.", num_valid2);
}

fn count_valid(entries: &Vec<&str>) -> usize {
    entries.iter().filter(|e| validate(e)).count()
}

fn validate(entry: &str) -> bool {
    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    required.iter().all(|s| entry.contains(s))
}

fn count_valid2(entries: &Vec<&str>) -> usize {
    entries.iter().filter(|e| validate2(e)).count()
}

fn validate2(entry: &str) -> bool {
    if !validate(entry) {
        return false;
    }

    let mut pairs = entry.split_ascii_whitespace();

    pairs.all(|p| match p.split(':').collect::<Vec<&str>>()[..] {
        ["byr", value] => value
            .parse::<u32>()
            .map_or(false, |byr| byr >= 1920 && byr <= 2002),
        ["iyr", value] => value
            .parse::<u32>()
            .map_or(false, |iyr| iyr >= 2010 && iyr <= 2020),
        ["eyr", value] => value
            .parse::<u32>()
            .map_or(false, |eyr| eyr >= 2020 && eyr <= 2030),
        ["hgt", value] => {
            value.len() > 2
                && match value.split_at(value.len() - 2) {
                    (hgt, "cm") => hgt
                        .parse::<u32>()
                        .map_or(false, |hgt| hgt >= 150 && hgt <= 193),
                    (hgt, "in") => hgt
                        .parse::<u32>()
                        .map_or(false, |hgt| hgt >= 59 && hgt <= 76),
                    _ => false,
                }
        }
        ["hcl", value] => {
            value.len() > 1
                && match value.split_at(1) {
                    ("#", hex) => hex.as_bytes().iter().all(|b| b.is_ascii_hexdigit()),
                    _ => false,
                }
        }
        ["ecl", value] => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
        ["pid", value] => value.len() == 9 && value.as_bytes().iter().all(|b| b.is_ascii_digit()),
        ["cid", _] => true,
        _ => false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate() {
        let entries = Vec::from([
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929",
            "hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm",
            "hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in",
        ]);

        assert_eq!(count_valid(&entries), 2);
    }

    #[test]
    fn test_validate2() {
        assert_eq!(
            validate2("eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"),
            false
        );
        assert_eq!(
            validate2("iyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946"),
            false
        );
        assert_eq!(
            validate2(
                "hcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"
            ),
            false
        );
        assert_eq!(
            validate2("hgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007"),
            false
        );
        assert_eq!(
            validate2("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f"),
            true
        );
        assert_eq!(
            validate2(
                "eyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"
            ),
            true
        );
        assert_eq!(
            validate2(
                "hcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022"
            ),
            true
        );
        assert_eq!(
            validate2("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"),
            true
        );
    }
}
