import json
import re

alphabet = "abcdefghijklmnopqrstuvwxyz"
argument_regex = re.compile(r"(?x)%(?:\((?P<key>\w+)\))?(?P<flags>[\#0\- +]*)?(?P<width>\*|\d+)?(?:.(?P<precision>\*|\d+))?[hlL]*(?P<type>[diouxXeEfFgGcrs%])")
plural_regex = re.compile(r"\#(\d+):?{\s?(\d+):([^\|]*)\|([^}]*)}")

with open("../../data/mmdb.json", "r") as f:
    stuff = json.load(f)

class nonlocl:
    # hack
    code = """\
use std::fmt::Display;

pub fn format_message(
    category_id: u32,
    instance_id: u32,
    arguments: Vec<Box<dyn Display>>,
) -> String {
    match (category_id, instance_id) {
"""
    current_args = ""
    i = 0

for category, items in stuff.items():
    for instance, string in items.items():
        nonlocl.current_args = ""
        nonlocl.i = 0
        nonlocl.code += f"""\
        ({category}, {instance}) => {{"""

        def replace(match: re.Match):
            flags = match.group("flags")
            type_ = match.group("type")
            width = int(w) if (w := match.group("width")) else w
            precision = int(p) if (p := match.group("precision")) else p

            if type_ == "%":
                return "%"

            nonlocl.code += f"\n            let {alphabet[nonlocl.i]} = &arguments[{nonlocl.i}];"
            nonlocl.current_args += f", {alphabet[nonlocl.i]}"
            nonlocl.i += 1

            if not width and not precision:
                return "{}"
            elif width and not precision:
                return f"{{:{flags}{width}}}"
            elif not width and precision:
                return f"{{:.{precision}}}"
            else:
                return f"{{:{flags}{width}.{precision}}}"

        new_format_string = re.sub(argument_regex, replace, string).replace("\r", "\\r").replace("\n", "\\n").replace('"', '\\\"')

        # Funcom's own format for plurals
        matches = re.findall(plural_regex, new_format_string)
        if matches:
            def replace(match: re.Match):
                return "TEMP"
            new_format_string = re.sub(plural_regex, replace, new_format_string)

        if nonlocl.current_args != "":
            nonlocl.code += f"\n            format!(\"{new_format_string}\"{nonlocl.current_args})\n        }}\n"
        else:
            nonlocl.code += f"\n            String::from(\"{new_format_string}\")\n        }}\n"

nonlocl.code += """\
        _ => panic!("Unknown MMDB entry"),
    }
}"""

print(nonlocl.code)
