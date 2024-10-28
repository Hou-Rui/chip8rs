mod asm;
mod chip8;
mod mem;

use qmetaobject::prelude::*;

qrc!(qml_resources, "qml" {
    "src/qml/main.qml" as "main.qml",
});

fn main() {
    let mut engine = QmlEngine::new();
    qml_register_type::<chip8::Chip8>(c"Chip8", 1, 0, c"Chip8");
    qml_resources();
    engine.load_file(QString::from("qrc:/qml/main.qml"));
    engine.exec();
}
