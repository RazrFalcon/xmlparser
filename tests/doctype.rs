extern crate xmlparser as xml;

#[macro_use] mod token;
use token::*;

test!(dtd_01, "<!DOCTYPE greeting SYSTEM \"hello.dtd\">",
    Token::EmptyDtd("greeting", Some(ExternalId::System("hello.dtd")))
);

test!(dtd_02, "<!DOCTYPE greeting PUBLIC \"hello.dtd\" \"goodbye.dtd\">",
    Token::EmptyDtd("greeting", Some(ExternalId::Public("hello.dtd", "goodbye.dtd")))
);

test!(dtd_03, "<!DOCTYPE greeting SYSTEM 'hello.dtd'>",
    Token::EmptyDtd("greeting", Some(ExternalId::System("hello.dtd")))
);

test!(dtd_04, "<!DOCTYPE greeting>",
    Token::EmptyDtd("greeting", None)
);

test!(dtd_05, "<!DOCTYPE greeting []>",
    Token::DtdStart("greeting", None),
    Token::DtdEnd
);

test!(dtd_06, "<!DOCTYPE greeting><a/>",
    Token::EmptyDtd("greeting", None),
    Token::ElementStart("", "a"),
    Token::ElementEnd(ElementEnd::Empty)
);

test!(dtd_entity_01,
"<!DOCTYPE svg [
    <!ENTITY ns_extend \"http://ns.adobe.com/Extensibility/1.0/\">
]>",
    Token::DtdStart("svg", None),
    Token::EntityDecl("ns_extend", EntityDefinition::EntityValue("http://ns.adobe.com/Extensibility/1.0/")),
    Token::DtdEnd
);

test!(dtd_entity_02,
"<!DOCTYPE svg [
    <!ENTITY Pub-Status \"This is a pre-release of the
specification.\">
]>",
    Token::DtdStart("svg", None),
    Token::EntityDecl("Pub-Status",
        EntityDefinition::EntityValue("This is a pre-release of the\nspecification.")
    ),
    Token::DtdEnd
);

test!(dtd_entity_03,
"<!DOCTYPE svg [
    <!ENTITY open-hatch SYSTEM \"http://www.textuality.com/boilerplate/OpenHatch.xml\">
]>",
    Token::DtdStart("svg", None),
    Token::EntityDecl("open-hatch",
        EntityDefinition::ExternalId(
            ExternalId::System("http://www.textuality.com/boilerplate/OpenHatch.xml")
        )
    ),
    Token::DtdEnd
);

test!(dtd_entity_04,
"<!DOCTYPE svg [
    <!ENTITY open-hatch
             PUBLIC \"-//Textuality//TEXT Standard open-hatch boilerplate//EN\"
             \"http://www.textuality.com/boilerplate/OpenHatch.xml\">
]>",
    Token::DtdStart("svg", None),
    Token::EntityDecl("open-hatch",
        EntityDefinition::ExternalId(
            ExternalId::Public(
                "-//Textuality//TEXT Standard open-hatch boilerplate//EN",
                "http://www.textuality.com/boilerplate/OpenHatch.xml"
            )
        )
    ),
    Token::DtdEnd
);

// TODO: NDATA will be ignored
test!(dtd_entity_05,
"<!DOCTYPE svg [
    <!ENTITY hatch-pic SYSTEM \"../grafix/OpenHatch.gif\" NDATA gif >
]>",
    Token::DtdStart("svg", None),
    Token::EntityDecl("hatch-pic",
        EntityDefinition::ExternalId(
            ExternalId::System("../grafix/OpenHatch.gif")
        )
    ),
    Token::DtdEnd
);

// TODO: unsupported data will be ignored
test!(dtd_entity_06,
"<!DOCTYPE svg [
    <!ELEMENT sgml ANY>
    <!ENTITY ns_extend \"http://ns.adobe.com/Extensibility/1.0/\">
    <!NOTATION example1SVG-rdf SYSTEM \"example1.svg.rdf\">
    <!ATTLIST img data ENTITY #IMPLIED>
]>",
    Token::DtdStart("svg", None),
    Token::EntityDecl("ns_extend",
        EntityDefinition::EntityValue("http://ns.adobe.com/Extensibility/1.0/")
    ),
    Token::DtdEnd
);

test!(dtd_err_01, "<!DOCTYPEEG[<!ENTITY%ETT\u{000a}SSSSSSSS<D_IDYT;->\u{000a}<",
    Token::Error("invalid token 'Doctype Declaration' at 1:1 cause expected space not 'E' at 1:10".to_string())
);

test!(dtd_err_02, "<!DOCTYPE s [<!ENTITY % name S YSTEM",
    Token::DtdStart("s", None),
    Token::Error("invalid token 'Doctype Entity Declaration' at 1:14 cause invalid ExternalID".to_string())
);
