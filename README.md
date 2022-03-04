# print-duration

The program takes an input specifying a duration and its units, and prints it
in an easy to read format.

## Examples

A duration in seconds:

    $ print-duration 8193s
    2h 16m 33s

A duration in minutes:

    $ print-duration 388m
    6h 28m

You can use more than one unit if required:

    $ print-duration '541 minutes 400 seconds'
    9h 7m 40s

Acceptable units are: ns, us, ms, sec, min, hours, days, weeks, months, years
(and few variations and abbreviations).