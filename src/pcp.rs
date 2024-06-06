use serde::Deserialize;
use std::collections::HashMap;
use chrono::{NaiveDate, Datelike};

#[derive(Deserialize, Debug)]
pub struct MateriaPrima {
    pub nome: String,
    pub custo: f64,
    pub tempo_entrega: i32,
}

#[derive(Deserialize, Debug)]
pub struct Produto {
    pub nome: String,
    pub materias_primas: Vec<(MateriaPrima, u32)>, // lista de materias primas e suas quantidades
    pub tempo_fabricacao: i32,
    pub capacidade_producao: i32, // adicionado campo de capacidade de produção
}

#[derive(Deserialize, Debug)]
pub struct Pedido {
    pub produto: Produto,
    pub data_entrega: String,
}

impl MateriaPrima {
    pub fn from_json(file_name: &str) -> Vec<MateriaPrima> {
        let materias_primas = std::fs::read_to_string(file_name).expect("Erro ao ler arquivo");
        let materias_primas: Vec<MateriaPrima> = serde_json::from_str(&materias_primas).expect("Erro ao desserializar Matéria Prima");
        materias_primas
    }
}

impl Produto {
    pub fn from_json(file_name: &str) -> Vec<Produto> {
        let produtos = std::fs::read_to_string(file_name).expect("Erro ao ler arquivo");
        let produtos: Vec<Produto> = serde_json::from_str(&produtos).expect("Erro ao desserializar Produto");
        produtos
    }
}

impl Pedido {
    pub fn from_json(file_name: &str) -> Vec<Pedido> {
        let pedidos = std::fs::read_to_string(file_name).expect("Erro ao ler arquivo Pedido");
        let pedidos: Vec<Pedido> = serde_json::from_str(&pedidos).expect("Erro ao desserializar");
        pedidos
    }
}

pub fn calcular_demanda(produtos: &Vec<Produto>, pedidos: &Vec<Pedido>) -> HashMap<String, u32> {
    let mut demanda: HashMap<String, u32> = HashMap::new();

    for pedido in pedidos {
        if let Some(produto) = produtos.iter().find(|p| p.nome == pedido.produto.nome) {
            for (materia_prima, quantidade) in &produto.materias_primas {
                *demanda.entry(materia_prima.nome.clone()).or_insert(0) += quantidade;
            }
        }
    }
    demanda
}

pub fn calcular_gasto_total(produtos: &Vec<Produto>, materias_primas: &HashMap<String, f64>) -> HashMap<String, f64> {
    let mut gasto_total_por_produto: HashMap<String, f64> = HashMap::new();

    for produto in produtos {
        let mut custo_total = 0.0;
        for (materia_prima, quantidade) in &produto.materias_primas {
            if let Some(custo) = materias_primas.get(&materia_prima.nome) {
                custo_total += custo * (*quantidade as f64);
            }
        }
        gasto_total_por_produto.insert(produto.nome.clone(), custo_total);
    }
    gasto_total_por_produto
}

pub fn calcular_lotes_compra(demanda: &HashMap<String, u32>) -> HashMap<String, u32> {
    let mut lotes_compra: HashMap<String, u32> = HashMap::new();

    for (materia_prima, quantidade) in demanda {
        let lote_compra = *quantidade; 
        lotes_compra.insert(materia_prima.clone(), lote_compra);
    }
    lotes_compra
}

pub fn calcular_data_pedido(data_entrega: &str, tempo_fabricacao: i32, tempos_entrega: &[i32]) -> String {
    let data_entrega_date = NaiveDate::parse_from_str(data_entrega, "%Y-%m-%d").unwrap();

    let tempo_total_meses = tempo_fabricacao + tempos_entrega.iter().cloned().max().unwrap_or(0);

    let mut ano = data_entrega_date.year();
    let mut mes = data_entrega_date.month() as i32;

    mes -= tempo_total_meses;

    while mes <= 0 {
        mes += 12;
        ano -= 1;
    }

    let dia = data_entrega_date.day();
    let data_pedido = NaiveDate::from_ymd(ano, mes as u32, dia);

    data_pedido.format("%Y-%m-%d").to_string()
}
