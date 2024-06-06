# Trabalho Rust PCP

Nome: Mateus Mendes Mattos
RA: 123117292

# Informações

# 1 - Apresentação do Problema Apresentar de forma sintética, a motivação do trabalho, qual é o problema abordado e o objetivo da pesquisa;

  O problema que motivou o trabalho é um problema de Planejamento e Controle de Produção. A ideia se baseia em otimizar a produçãode um fábrica, utilizando de técnicas que foram aprendidas em sala. O desafio é criar um algoritmo Rust que consiga suprir com todas as demandas dessa fábrica. 

# 2 - Complexidade do Problema Utilizando a análise de complexidade de problemas, classificar o problema quanto à sua complexidade de resolução, se P. NP ou NP-Completo, justificando sua resposta, com base na teoria e exemplos;
  Este problema trata-se de um problema NP-Completo. Para definir se um problema é um problema da classe NP, deve-se seguir este conceito:  A classe NP contém os problemas que podem ser verificados por um algoritmo polinomial, ou seja, um algoritmo que consome um tempo limitado por um polinômio no tamanho da entrada e da solução.

Entretanto, os problemas NP-Completos implicam que o problema não pode ser resolvido em tempo polinomial. O problema de Planejamento e controle de produção não pode ser resolvido em tempo polinomial, ou seja, não é possível encontrar uma solução ótima para este problema, atendendo todas as restrições. Devido aos números de combinações possíveis, incertezas, restrições complexas que resultam em um problema da classe NP-Completo.

# 3 - Algoritmos conhecidos Pesquisar os algoritmos conhecidos que resolvam o problema em questão, tentando, ao máximo, colocar características de cada um;
 Algoritmos gulosos - 

* Tomam decisões locais que parecem ser as melhores em cada etapa.

* Não revisam ou alteram decisões tomadas anteriormente.

* São simples de implementar e geralmente têm complexidade computacional mais baixa.

* Não garantem uma solução ótima, mas podem ser eficazes em muitos casos práticos.

* Exemplos incluem o algoritmo de Kruskal para árvores de custo mínimo e o algoritmo de Dijkstra para caminhos mínimos em grafos.

Programação Linear - 

* Formula o problema em termos de variáveis de decisão, uma função objetivo linear e um conjunto de restrições lineares.

* É usado para resolver problemas de otimização linear, onde todas as restrições e a função objetivo são lineares.

* Pode ser resolvido eficientemente usando métodos como o método simplex ou a programação linear inteira.

* Fornece uma solução ótima, desde que o problema atenda às condições de linearidade.

* Amplamente utilizado em uma variedade de campos, incluindo economia, engenharia e logística.

Heurística - 

* São métodos de solução que não garantem uma solução ótima, mas tentam encontrar uma solução aceitável em um tempo razoável.

* São úteis para problemas complexos, onde encontrar a solução ótima é impraticável em termos de tempo ou recursos.

* Podem ser baseados em regras, construtivos, de busca local, entre outros.

* Podem ser adaptados para diferentes problemas e contextos específicos.

* Exemplos incluem a heurística do vizinho mais próximo e algoritmos genéticos.

Programação dinâmica - 

* Divide o problema em subproblemas sobrepostos e resolve cada subproblema apenas uma vez.

* Armazena as soluções dos subproblemas em uma tabela para evitar recálculo.

* É usado para resolver problemas de otimização que exibem a propriedade de subestrutura ótima e superposição de subproblemas.

* Geralmente é aplicável a problemas de otimização combinatória.

* Fornece uma solução ótima, mas pode ser computacionalmente intensivo em alguns casos.

* Exemplos incluem o algoritmo de Floyd-Warshall para o problema do caminho mais curto e o algoritmo de Knapsack para o problema da mochila.

# 4 - Algoritmo escolhido e implementação Apontar o algoritmo escolhido, indicando a fonte de onde tirou o mesmo, implementar e versionar no GitHub;

  Não escolhi nenhum algoritmo para resolução desse problema. Utilizei métodos, parecidos com a programação dinâmica para resolver.

  Decidi implementar funções que calculassem o que era necessário e depois me retornam um resultado, exemplo de funções com "calcular_data_pedido".
  Essa função calcula quando deve ser feito um pedido, considerando o menor tempo possível para que os produtos fiquem em estoque, fazendo assim com que a entrega seja realizada com sucesso. 

# 5 - Complexidade do algoritmo escolhido Utilizando o método de contagem simples, classificar com a classificação Big-O, o algoritmo, destacando o trecho de maior complexidade;

  Acredito que por não usar um algoritmo específico, posso calcular o big-O de todas as funções e destacar os trechos de maior complexidade de cada função, separadamente. 


# 6 - Paradigma e Estratégia do algoritmo escolhido Evidenciar, analisar e apresentar quais estratégias o algoritmo escolhido utiliza;

  A estratégia utilizada foi dividir as restrições em subproblemas, para que as tarefas fossem tratadas individualemente.  

# 7 - Comparação com, pelo menos mais um algoritmo Selecionar outro algoritmo, que não o escolhido e comparar em complexidade e paradigma

  O que mais se asemelha, são algoritmos de programação dinâmica. Um algoritmo que pode ser escolhido é: Algoritmo de Knapsack
  
  A complexidade de tempo é determinada pelo número de subproblemas que precisam ser resolvidos e pelo tempo necessário para resolver cada subproblema. 
  a complexidade de tempo para a solução por programação dinâmica é O(nc). 

# 8 - Linguagem Explique como o Rust foi utilizado no desenvolvimento do algoritmo;

  O rust teve um papel fundamental para o desenvolvimento do meu algoritmo. As bibliotecas usadas são muito boas e a forma como o Rust trata os dados é muito impressionante.
  Utilizei a biblioteca de HashMap: onde precisei usar uma chave e um valor, Chrono: Para conseguir tratar datas, serde: para conseguir desserializar os arquivos json. 
