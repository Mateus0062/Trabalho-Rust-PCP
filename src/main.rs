//use crate::pcp::calcular_proximo_pedido;
use crate::pcp::calcular_data_pedido;
use crate::pcp::{MateriaPrima, Pedido, Produto, calcular_demanda, calcular_lotes_compra, calcular_gasto_total};
use std::collections::HashMap;
mod pcp;
fn main() {
    println!("Sistema de Planejamento e Controle de Produção (PCP)");

    let materias_primas = MateriaPrima::from_json("materias_primas.json");
    println!("Matérias primas: {:?}", materias_primas);

    println!();

    let produtos = Produto::from_json("produtos.json");
    println!("Produtos: {:?}", produtos);

    println!();

    let pedidos = Pedido::from_json("pedidos.json");
    println!("Pedidos: {:?}", pedidos);

    println!();

    let demanda = calcular_demanda(&produtos, &pedidos);
    println!("Demanda: {:?}", demanda);

    println!();

    let lotes_compra = calcular_lotes_compra(&demanda);
    println!("Lotes de Compra: {:?}", lotes_compra);

   let mut materias_primas_map = HashMap::new();
    for materia_prima in materias_primas {
        materias_primas_map.insert(materia_prima.nome.clone(), materia_prima.custo);
    }

    println!();

    let gasto_total = calcular_gasto_total(&produtos, &materias_primas_map);
    println!("Gasto Total por Produto: {:?}", gasto_total);

    println!();

    for pedido in &pedidos {
        let produto = &pedido.produto;
        let tempos_entrega_materias_primas: Vec<i32> = produto.materias_primas.iter().map(|(mp, _)| mp.tempo_entrega).collect();
        let data_pedido = calcular_data_pedido(
            &pedido.data_entrega,
            produto.tempo_fabricacao,
            &tempos_entrega_materias_primas,
        );
        println!(
            "Produto: {}, Data de Entrega: {}, Data do Pedido: {}",
            produto.nome, pedido.data_entrega, data_pedido
        );
    }
}