pub const PROPERTY1: &str =
r#"Formula
+-> always = always
    +-> -> = ->
        +-> = = =
        |   +-> % = %
        |   |   +-> TIME = t
        |   |   +-> TIMEUNIT = h
        |   |       +-> NUMBER = 24
        |   +-> NUMBER = 0
        +-> always = always
            +-> Interval
            |   +-> TIMEUNIT = h
            |   |   +-> NUMBER = 0
            |   +-> TIMEUNIT = h
            |       +-> NUMBER = 24
            +-> < = <
                +-> sumtime = sumtime
                |   +-> TIMEUNIT = s
                |   |   +-> NUMBER = 5
                |   +-> * = *
                |       +-> NUMBER = 1
                |       +-> power = power
                +-> POWERUNIT = kwh
                    +-> NUMBER = 10
"#;

pub const PROPERTY2: &str =
r#"Formula
+-> eventually = eventually
    +-> ! = !
    +-> > = >
        +-> count = count
        |   +-> NUMBER = 1
        +-> NUMBER = 5
"#;

pub const PROPERTY3: &str =
r#"Formula
+-> always = always
    +-> foreach = foreach
        +-> -> = ->
            +-> NUMBER = 1
            +-> eventually = eventually
                +-> Interval
                |   +-> TIMEUNIT = h
                |   |   +-> NUMBER = 0
                |   +-> TIMEUNIT = h
                |       +-> NUMBER = 6
                +-> ! = !
                    +-> NUMBER = 1
"#;

pub const PROPERTY4: &str =
r#"Formula
+-> always = always
    +-> count = count
        +-> & = &
            +-> = = =
            |   +-> name = name
            |   +-> STRING = fridge
            +-> NUMBER = 1
"#;

pub const PROPERTY5: &str =
r#"Formula
+-> always = always
    +-> -> = ->
        +-> >= = >=
        |   +-> count = count
        |   |   +-> NUMBER = 1
        |   +-> NUMBER = 5
        +-> eventually = eventually
            +-> Interval
            |   +-> TIMEUNIT = h
            |   |   +-> NUMBER = 0
            |   +-> TIMEUNIT = h
            |       +-> NUMBER = 6
            +-> < = <
                +-> count = count
                |   +-> NUMBER = 1
                +-> NUMBER = 5
"#;

pub const PROPERTY6: &str =
r#"Formula
+-> always = always
    +-> <= = <=
        +-> sum = sum
        |   +-> * = *
        |       +-> NUMBER = 1
        |       +-> power = power
        +-> POWERUNIT = w
            +-> NUMBER = 100
"#;

pub const PROPERTY7: &str =
r#"Formula
+-> always = always
|   +-> NUMBER = 7
+-> always = always
    +-> NUMBER = 7
"#;

