#!/usr/bin/env python3
import operator


def get_passports(raw_strings):
    passports = []
    for passport_str in raw_strings:
        pairs = passport_str.split()
        passport = {k: v for k, v in [pair.split(":") for pair in pairs]}
        passports.append(passport)
    return passports


class InvalidPassportException(Exception):
    pass


class Passport:
    valid_fields = {
        "byr",  # (Birth Year)
        "iyr",  # (Issue Year)
        "eyr",  # (Expiration Year)
        "hgt",  # (Height)
        "hcl",  # (Hair Color)
        "ecl",  # (Eye Color)
        "pid",  # (Passport ID)
        # "cid"   # (Country ID is IGNORED)
    }

    def __init__(self, **kwargs):
        if not self.validate_keys(kwargs):
            raise InvalidPassportException("missing fields ", self.valid_fields - set(kwargs.keys()))
        self.byr = kwargs.get("byr", None)
        self.iyr = kwargs.get("iyr", None)
        self.eyr = kwargs.get("eyr", None)
        self.hgt = kwargs.get("hgt", None)
        self.hcl = kwargs.get("hcl", None)
        self.ecl = kwargs.get("ecl", None)
        self.pid = kwargs.get("pid", None)

    byr = property(operator.attrgetter("_byr"))
    iyr = property(operator.attrgetter("_iyr"))
    eyr = property(operator.attrgetter("_eyr"))
    hgt = property(operator.attrgetter("_hgt"))
    hcl = property(operator.attrgetter("_hcl"))
    ecl = property(operator.attrgetter("_ecl"))
    pid = property(operator.attrgetter("_pid"))

    @byr.setter
    def byr(self, b: str):
        if not b or not b.isdigit() or not int(b) in range(1920, 2003):
            raise InvalidPassportException("invalid byr ", b)
        self._byr = int(b)

    @iyr.setter
    def iyr(self, i: str):
        if not i or not i.isdigit() or not int(i) in range(2010, 2021):
            raise InvalidPassportException("invalid iyr ", i)
        self._iyr = int(i)

    @eyr.setter
    def eyr(self, ey: str):
        if not ey or not ey.isdigit() or not int(ey) in range(2020, 2031):
            raise InvalidPassportException("invalid eyr ", ey)
        self._eyr = int(ey)

    @hgt.setter
    def hgt(self, h: str):
        if not h or not (h.endswith("cm") or h.endswith("in")):
            raise InvalidPassportException("invalid hgt no unit ", h)
        measurement, unit = h[:-2], h[-2:]
        if not unit or not measurement or not measurement.isdigit():
            raise InvalidPassportException("invalid hgt unit, meas ", h)
        if unit == "cm" and not int(measurement) in range(150, 194):
            raise InvalidPassportException("invalid hgt, cm range ", h)
        elif unit == "in" and not int(measurement) in range(59, 77):
            raise InvalidPassportException("invalid hgt, inch range ", h)
        self._hgt = h

    @hcl.setter
    def hcl(self, hc):
        if not hc or not hc.startswith("#") or not len(hc) == 7:
            raise InvalidPassportException("invalid hc, ", hc)
        try:
            dec = int(hc[1:], 16)
            if not dec <= 16777215:
                raise InvalidPassportException("invalid hc, hex amt", dec)
        except ValueError:
            raise InvalidPassportException("invalid hc, not hex", hc)
        self._hcl = hc

    @ecl.setter
    def ecl(self, ec):
        if ec not in {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"}:
            raise InvalidPassportException("invalid ecl", ec)
        self._ecl = ec

    @pid.setter
    def pid(self, p):
        if not p or not p.isdigit() or not len(p) == 9:
            raise InvalidPassportException("invalid pid ", p)
        self._pid = p

    @classmethod
    def validate_keys(cls, candidate: dict) -> bool:
        candidate.pop("cid", None)
        if cls.valid_fields - set(candidate.keys()):
            return False
        return True


if __name__ == "__main__":
    with open("../data_files/day04_input.txt", "r") as f:
        data = f.read().split("\n\n")
        strings = list(map(lambda s: s.replace("\n", " "), data))
        passports = get_passports(strings)

    # PART 1
    part_1 = sum([1 if Passport.validate_keys(pport) else 0 for pport in passports])
    print("PART 1: ", part_1)
    # PART 2
    full_valid = 0
    for pport in get_passports(strings):
        try:
            p = Passport(**pport)
            full_valid += 1
        except InvalidPassportException as ipe:
            # print(f"{ipe.args[0]}: {ipe.args[1]}")
            continue
    print("PART 2: ", full_valid)
