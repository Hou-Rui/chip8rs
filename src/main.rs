mod asm;
mod cpu;
mod mem;

use qmetaobject::prelude::*;

qrc!(qml_resources, "qml" {
    "src/qml/main.qml" as "main.qml",
});

fn main() {
    qml_register_type::<cpu::Cpu>(c"Cpu", 1, 0, c"Cpu");
    qml_resources();
    let mut engine = QmlEngine::new();
    engine.load_file(QString::from("qrc:/qml/main.qml"));
    engine.exec();
}
