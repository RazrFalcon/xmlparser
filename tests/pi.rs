extern crate xmlparser as xml;

#[macro_use] mod token;
use token::*;

test!(pi_01, "<?xslt ma?>",
    Token::PI("xslt", Some("ma"))
);

test!(pi_02, "<?xslt \t\n m?>",
    Token::PI("xslt", Some("m"))
);

test!(pi_03, "<?xslt?>",
    Token::PI("xslt", None)
);

test!(pi_04, "<?xslt ?>",
    Token::PI("xslt", None)
);

test!(pi_05, "<?xml-stylesheet?>",
    Token::PI("xml-stylesheet", None)
);

test!(pi_err_01, "<??xml \t\n m?>",
    Token::Error("invalid token 'Processing Instruction' at 1:1 cause invalid name token".to_string())
);

test!(declaration_01, "<?xml version=\"1.0\"?>",
    Token::Declaration("1.0", None, None)
);

test!(declaration_02, "<?xml version='1.0'?>",
    Token::Declaration("1.0", None, None)
);

test!(declaration_03, "<?xml version='1.0' encoding=\"UTF-8\"?>",
    Token::Declaration("1.0", Some("UTF-8"), None)
);

test!(declaration_04, "<?xml version='1.0' encoding='UTF-8'?>",
    Token::Declaration("1.0", Some("UTF-8"), None)
);

test!(declaration_05, "<?xml version='1.0' encoding='utf-8'?>",
    Token::Declaration("1.0", Some("utf-8"), None)
);

test!(declaration_06, "<?xml version='1.0' encoding='EUC-JP'?>",
    Token::Declaration("1.0", Some("EUC-JP"), None)
);

test!(declaration_07, "<?xml version='1.0' encoding='UTF-8' standalone='yes'?>",
    Token::Declaration("1.0", Some("UTF-8"), Some("yes"))
);

test!(declaration_08, "<?xml version='1.0' encoding='UTF-8' standalone='no'?>",
    Token::Declaration("1.0", Some("UTF-8"), Some("no"))
);

test!(declaration_09, "<?xml version='1.0' standalone='no'?>",
    Token::Declaration("1.0", None, Some("no"))
);

// Declaration with an invalid order
test!(declaration_err_01, "<?xml encoding='UTF-8' version='1.0'?>",
    Token::Error("invalid token 'Declaration' at 1:1 cause expected 'version' not 'encodin' at 1:7".to_string())
);

test!(declaration_err_02, "<?xml version='1.0' encoding='*invalid*'?>",
    Token::Error("invalid token 'Declaration' at 1:1 cause expected quote mark not '*' at 1:31".to_string())
);

test!(declaration_err_03, "<?xml version='2.0'?>",
    Token::Error("invalid token 'Declaration' at 1:1 cause expected '1.' not '2.' at 1:16".to_string())
);

test!(declaration_err_04, "<?xml version='1.0' standalone='true'?>",
    Token::Error("invalid token 'Declaration' at 1:1 cause expected 'yes', 'no' not 'true' at 1:33".to_string())
);

test!(declaration_err_05, "<?xml version='1.0' yes='true'?>",
    Token::Error("invalid token 'Declaration' at 1:1 cause expected '?>' not 'ye' at 1:21".to_string())
);

test!(declaration_err_06, "<?xml version='1.0' encoding='UTF-8' standalone='yes' yes='true'?>",
    Token::Error("invalid token 'Declaration' at 1:1 cause expected '?>' not 'ye' at 1:55".to_string())
);

test!(declaration_err_07, "\u{000a}<?xml\u{001d}\u{000a}\u{0000}&jg'];",
    Token::Error("invalid token 'Processing Instruction' at 2:1 cause expected '?>' not '\u{1d}\n' at 2:6".to_string())
);

test!(declaration_err_08, "<?xml \t\n ?m?>",
    Token::Error("invalid token 'Declaration' at 1:1 cause expected 'version' not '?m?>' at 2:2".to_string())
);

test!(declaration_err_09, "<?xml \t\n m?>",
    Token::Error("invalid token 'Declaration' at 1:1 cause expected 'version' not 'm?>' at 2:2".to_string())
);
