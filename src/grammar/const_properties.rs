pub const PROPERTY1: &str =
r#"Program
+-> Property
    +-> always = always
        +-> -> = ->
            +-> = = =
            |   +-> ( = (
            |   +-> % = %
            |   |   +-> TIME = t
            |   |   +-> UNIT = h
            |   |       +-> NUMBER = 24
            |   +-> NUMBER = 0
            |   +-> ) = )
            +-> < = <
                +-> always = always
                |   +-> Interval
                |   |   +-> UNIT = h
                |   |   |   +-> NUMBER = 0
                |   |   +-> UNIT = h
                |   |       +-> NUMBER = 24
                |   +-> sumtime = sumtime
                |       +-> * = *
                |           +-> active = active
                |           +-> power = power
                +-> UNIT = kWh
                    +-> NUMBER = 10
"#;

pub const PROPERTY2: &str =
r#"Program
+-> Property
    +-> eventually = eventually
        +-> not = not
        +-> > = >
            +-> count = count
            |   +-> active = active
            +-> NUMBER = 5
"#;

pub const PROPERTY3: &str =
r#"Program
+-> Property
    +-> always = always
        +-> foreach = foreach
            +-> -> = ->
                +-> active = active
                +-> eventually = eventually
                    +-> Interval
                    |   +-> UNIT = h
                    |   |   +-> NUMBER = 0
                    |   +-> UNIT = h
                    |       +-> NUMBER = 6
                    +-> ! = !
                        +-> active = active
"#;

pub const PROPERTY4: &str =
r#"Program
+-> Property
    +-> always = always
        +-> count = count
            +-> & = &
                +-> = = =
                |   +-> name = name
                |   +-> STRING = fridge
                +-> active = active
"#;

pub const PROPERTY5: &str =
r#"Program
+-> Property
    +-> always = always
        +-> -> = ->
            +-> >= = >=
            |   +-> count = count
            |   |   +-> active = active
            |   +-> NUMBER = 5
            +-> < = <
                +-> eventually = eventually
                |   +-> Interval
                |   |   +-> UNIT = h
                |   |   |   +-> NUMBER = 0
                |   |   +-> UNIT = h
                |   |       +-> NUMBER = 6
                |   +-> count = count
                |       +-> active = active
                +-> NUMBER = 5
"#;

pub const PROPERTY6: &str =
r#"Program
+-> Property
    +-> always = always
        +-> <= = <=
            +-> sum = sum
            |   +-> * = *
            |       +-> active = active
            |       +-> power = power
            +-> UNIT = w
                +-> NUMBER = 100
"#;

pub const PROPERTY7: &str =
r#"Program
+-> Property
    +-> always = always
    |   +-> NUMBER = 7
    +-> eventually = eventually
        +-> NUMBER = 7
"#;

pub const PROPERTY8: &str =
r#"Program
+-> Property
    +-> until = until
        +-> not = not
        +-> ( = (
        +-> active = active
        +-> , = ,
        +-> NUMBER = 10
        +-> ) = )
"#;