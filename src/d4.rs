use crate::util;
use regex::Regex;
use std::collections::HashMap;
use std::vec::Vec;

#[derive(Clone, Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

fn validate_hgt(h: &str) -> bool {
    let re = Regex::new(r"(?P<height>\d+)(?P<unit>in|cm)");
    if re.is_err() {
        return false;
    }
    re.unwrap()
        .captures(h)
        .map(|caps| {
            let height = caps["height"].parse::<i32>().unwrap_or(0);
            let unit = &caps["unit"];
            (unit == "cm" && height >= 150 && height <= 193)
                || (unit == "in" && height >= 59 && height <= 76)
        })
        .unwrap_or(false)
}

impl Passport {
    fn valid(&self) -> bool {
        let conditions = vec![
            self.byr
                .parse::<i32>()
                .map(|i| i >= 1920 && i <= 2002)
                .unwrap_or(false),
            self.iyr
                .parse::<i32>()
                .map(|i| i >= 2010 && i <= 2020)
                .unwrap_or(false),
            self.eyr
                .parse::<i32>()
                .map(|i| i >= 2020 && i <= 2030)
                .unwrap_or(false),
            validate_hgt(&self.hgt),
            Regex::new(r"^#[0-9a-f]{6}$")
                .map(|r| r.is_match(&self.hcl))
                .unwrap_or(false),
            vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&self.ecl.as_str()),
            Regex::new(r"^\d{9}$")
                .map(|r| r.is_match(&self.pid))
                .unwrap_or(false),
        ];
        conditions.iter().all(|x| *x)
    }
}

#[cfg(test)]
mod tests {
    use crate::d4::Passport;
    #[test]
    fn invalid_passports() {
        assert_eq!(
            Passport {
                eyr: String::from("1972"),
                cid: Some(String::from("100")),
                hcl: String::from("#18171d"),
                ecl: String::from("amb"),
                hgt: String::from("170"),
                pid: String::from("186cm"),
                iyr: String::from("2018"),
                byr: String::from("1926"),
            }
            .valid(),
            false
        );

        assert_eq!(
            Passport {
                eyr: String::from("1967"),
                cid: None,
                hcl: String::from("#602927"),
                ecl: String::from("grn"),
                hgt: String::from("170cm"),
                pid: String::from("012533040"),
                iyr: String::from("2019"),
                byr: String::from("1946"),
            }
            .valid(),
            false
        );

        assert_eq!(
            Passport {
                eyr: String::from("2020"),
                cid: Some(String::from("277")),
                hcl: String::from("dab227"),
                ecl: String::from("brn"),
                hgt: String::from("182cm"),
                pid: String::from("021572410"),
                iyr: String::from("2012"),
                byr: String::from("1992"),
            }
            .valid(),
            false
        );

        assert_eq!(
            Passport {
                eyr: String::from("2038"),
                cid: Some(String::from("100")),
                hcl: String::from("74454a"),
                ecl: String::from("zzz"),
                hgt: String::from("170"),
                pid: String::from("3556412378"),
                iyr: String::from("2023"),
                byr: String::from("2007"),
            }
            .valid(),
            false
        );
        assert_eq!(
            Passport {
                eyr: String::from("2038"),
                cid: Some(String::from("100")),
                hcl: String::from("#74454a"),
                ecl: String::from("brn"),
                hgt: String::from("170cm"),
                pid: String::from("355612378"),
                iyr: String::from("2020"),
                byr: String::from("2001"),
            }
            .valid(),
            false
        );
        assert_eq!(
            Passport {
                eyr: String::from("2030"),
                cid: Some(String::from("100")),
                hcl: String::from("74454a"),
                ecl: String::from("brn"),
                hgt: String::from("170cm"),
                pid: String::from("355612378"),
                iyr: String::from("2020"),
                byr: String::from("2001"),
            }
            .valid(),
            false
        );
        assert_eq!(
            Passport {
                eyr: String::from("2030"),
                cid: Some(String::from("100")),
                hcl: String::from("#74454a"),
                ecl: String::from("xyz"),
                hgt: String::from("170cm"),
                pid: String::from("355612378"),
                iyr: String::from("2020"),
                byr: String::from("2001"),
            }
            .valid(),
            false
        );
        assert_eq!(
            Passport {
                eyr: String::from("2030"),
                cid: Some(String::from("100")),
                hcl: String::from("#74454a"),
                ecl: String::from("brn"),
                hgt: String::from("170in"),
                pid: String::from("355612378"),
                iyr: String::from("2020"),
                byr: String::from("2001"),
            }
            .valid(),
            false
        );
        assert_eq!(
            Passport {
                eyr: String::from("2030"),
                cid: Some(String::from("100")),
                hcl: String::from("#74454a"),
                ecl: String::from("brn"),
                hgt: String::from("23cm"),
                pid: String::from("355612378"),
                iyr: String::from("2020"),
                byr: String::from("2001"),
            }
            .valid(),
            false
        );
        assert_eq!(
            Passport {
                eyr: String::from("2030"),
                cid: Some(String::from("100")),
                hcl: String::from("#74454a"),
                ecl: String::from("brn"),
                hgt: String::from("170cm"),
                pid: String::from("3556123789"),
                iyr: String::from("2020"),
                byr: String::from("2001"),
            }
            .valid(),
            false
        );
        assert_eq!(
            Passport {
                eyr: String::from("2030"),
                cid: Some(String::from("100")),
                hcl: String::from("#74454a"),
                ecl: String::from("brn"),
                hgt: String::from("170cm"),
                pid: String::from("355612378"),
                iyr: String::from("2023"),
                byr: String::from("2001"),
            }
            .valid(),
            false
        );
        assert_eq!(
            Passport {
                eyr: String::from("2030"),
                cid: Some(String::from("100")),
                hcl: String::from("#74454a"),
                ecl: String::from("brn"),
                hgt: String::from("170cm"),
                pid: String::from("355612378"),
                iyr: String::from("2020"),
                byr: String::from("1845"),
            }
            .valid(),
            false
        );
    }
}

fn parse_passport(s: &str) -> Option<Passport> {
    let re = Regex::new(r"(?P<field>\w+):(?P<value>[^\s]+)").unwrap();
    let mut fields: HashMap<String, String> = HashMap::new();
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    re.captures_iter(s).for_each(|c| {
        fields.insert(c["field"].to_string(), c["value"].to_string());
    });
    if required_fields
        .iter()
        .all(|f| fields.contains_key(&f.to_string()))
    {
        Some(Passport {
            byr: fields["byr"].clone(),
            iyr: fields["iyr"].clone(),
            eyr: fields["eyr"].clone(),
            hgt: fields["hgt"].clone(),
            hcl: fields["hcl"].clone(),
            ecl: fields["ecl"].clone(),
            pid: fields["pid"].clone(),
            cid: fields.get("cid").cloned(),
        })
    } else {
        None
    }
}

fn parse_passports() -> Vec<Option<Passport>> {
    let lines = util::file_lines("./data/d4.txt");
    let partitioned = util::partition_by_blank_lines(lines);
    partitioned
        .iter()
        .map(|strings| strings.join(" "))
        .map(|s| parse_passport(&s))
        .collect()
}

fn apply_extra_validation(passport: &Option<Passport>) -> Option<Passport> {
    passport
        .as_ref()
        .and_then(|pp| if pp.valid() { Some(pp.clone()) } else { None })
}

#[allow(dead_code)]
pub fn solve() {
    println!("Part 1");
    println!(
        "{}",
        parse_passports().iter().filter(|p| p.is_some()).count()
    );
    println!("Part 2");
    println!(
        "{}",
        parse_passports()
            .iter()
            .map(apply_extra_validation)
            .filter(|p| p.is_some())
            .count()
    );
}
