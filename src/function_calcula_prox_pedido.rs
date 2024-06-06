use chrono::{NaiveDate, Datelike};

fn calcular_proximo_pedido(data_entrega: &str, tempo_fabricacao_meses: i32, tempo_entrega_meses1: i32, tempo_entrega_meses2: i32) -> String {
    let data_entrega_date = NaiveDate::parse_from_str(data_entrega, "%d/%m/%Y").unwrap();

    let tempo_total_meses = tempo_fabricacao_meses + std::cmp::max(tempo_entrega_meses1, tempo_entrega_meses2);

    let mut ano = data_entrega_date.year();
    let mut mes = data_entrega_date.month() as i32;

    mes -= tempo_total_meses;

    while mes <= 0 {
        mes += 12;
        ano -= 1;
    }

    let dia = data_entrega_date.day();
    let data_proximo_pedido = NaiveDate::from_ymd(ano, mes as u32, dia);

    data_proximo_pedido.format("%d/%m/%Y").to_string()
}

fn main() {
    let data_entrega = "01/06/2024";
    let tempo_fabricacao_meses = 3;  // Tempo de fabricação de 3 meses
    let tempo_entrega_meses1 = 1;  // Tempo de entrega de 1 mês para matéria-prima 1
    let tempo_entrega_meses2 = 2;  // Tempo de entrega de 2 meses para matéria-prima 2

    // Calcular a data do próximo pedido
    let proximo_pedido = calcular_proximo_pedido(data_entrega, tempo_fabricacao_meses, tempo_entrega_meses1, tempo_entrega_meses2);

    println!("Data do próximo pedido: {}", proximo_pedido);

    let data_entrega1 = "01/11/2024";
    let tempo_fabricacao_meses1 = 3;  // Tempo de fabricação de 3 meses
    let tempo_entrega_meses11 = 1;  // Tempo de entrega de 1 mês para matéria-prima 1
    let tempo_entrega_meses21 = 2;  // Tempo de entrega de 2 meses para matéria-prima 2

    // Calcular a data do próximo pedido
    let proximo_pedido1 = calcular_proximo_pedido(data_entrega1, tempo_fabricacao_meses1, tempo_entrega_meses11, tempo_entrega_meses21);

    println!("Data do próximo pedido: {}", proximo_pedido1);

    let data_entrega2 = "01/07/2024";
    let tempo_fabricacao_meses2 = 4;  // Tempo de fabricação de 3 meses
    let tempo_entrega_meses12 = 3;  // Tempo de entrega de 1 mês para matéria-prima 3
    let tempo_entrega_meses22 = 4;  // Tempo de entrega de 2 meses para matéria-prima 4

    // Calcular a data do próximo pedido
    let proximo_pedido2 = calcular_proximo_pedido(data_entrega2, tempo_fabricacao_meses2, tempo_entrega_meses12, tempo_entrega_meses22);

    println!("Data do próximo pedido: {}", proximo_pedido2);
}
