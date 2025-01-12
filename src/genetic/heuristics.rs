use kambo_graph::{graphs::simple::UndirectedGraph, Graph, GraphMut};
use rand::seq::IteratorRandom;

use super::Chromosome;

pub fn h1(graph: &UndirectedGraph<usize>) -> Option<Chromosome> {
    // Inicializa um vetor de genes com valores 0.
    // O tamanho do vetor é igual ao número de vértices no grafo.
    let mut genes = vec![0u8; graph.order()];

    // Faz uma cópia do grafo original para ser manipulado sem alterar o original.
    let mut h = graph.clone();

    // Cria um gerador de números aleatórios para escolher vértices aleatoriamente.
    let mut rng = rand::thread_rng();

    // Enquanto o grafo h ainda tiver vértices...
    while let Some(v) = h.vertices().choose(&mut rng).cloned() {
        // Passo 4: Define f(v) = 2, marcando o vértice v com a cor 2.
        genes[v] = 2;

        // Obtém os vizinhos de v no grafo `h`.
        let neighbors: Vec<usize> = h
            .neighbors(&v)
            .map(|n| n.cloned().collect())
            .unwrap_or_default();

        // Passo 5: Se v tem vizinhos, escolha um (o primeiro da lista) e defina f(u) = 1.
        if let Some(first_neighbor) = neighbors.first() {
            genes[*first_neighbor] = 1;

            // Passo 6: Para os demais vizinhos de v, define f(w) = 0.
            for w in neighbors.iter().skip(1) {
                genes[*w] = 0;
            }
        }

        // Passo 7: Remove o vértice `v` e seus vizinhos do grafo `h`.
        let _ = h.remove_vertex(&v);
        for neighbor in neighbors {
            let _ = h.remove_vertex(&neighbor);
        }

        // Passo 8: Enquanto houver vértices isolados em h...
        let isolated_vertices = h.get_isolated_vertices();
        for z in isolated_vertices {
            // Caso contrário, define f(z) = 1.
            genes[z] = 1;
            let has_neighbor_with_1 = graph
                .neighbors(&z)
                .map(|mut neighbors| neighbors.any(|n| genes[*n] == 1))
                .unwrap_or(false);

            // Verifica se `z` tem vizinhos no grafo original com f = 1.
            if !has_neighbor_with_1 {
                // Se não há vizinhos com f = 1, escolhe um vizinho com f = 0 e define f = 1.
                if let Some(mut neighbors) = graph.neighbors(&z) {
                    if let Some(first) = neighbors.find(|&n| genes[*n] == 0) {
                        genes[*first] = 1;
                    }
                }
            }

            // Passo 12: Remove o vértice `z` do grafo `h`.
            let _ = h.remove_vertex(&z);
        }
    }

    // Retorna a solução como um Chromosome, encapsulando o vetor de genes.
    Some(Chromosome::new(genes))
}

/*
    Tem comportamente não determinisco, por uma questão de otimização o rust
    não garante que os iteradores vão retornar os elementos sempre na mesma ordem
    O método neghbors pode retornar os vizinhos em uma ordem que não é garantida ser consistente entre execuções,
    já que a implementação interna da estrutura de dados (como HashMap ) pode não preservar a ordem de inserção ou retorno.
*/
pub fn h2(graph: &UndirectedGraph<usize>) -> Option<Chromosome> {
    // Inicializa um vetor de genes com valores 0.
    // O tamanho do vetor é igual ao número de vértices no grafo.
    let mut genes = vec![0u8; graph.order()];

    // Faz uma cópia do grafo original para ser manipulado sem alterar o original.
    let mut h = graph.clone();

    // Enquanto o grafo h ainda tiver vértices... (Já captura o v = vértice de maior grau do grafo)
    while let Some(v) = h.vertices().max_by_key(|&vertex| h.degree(vertex)).cloned() {
        // Passo 4: Define f(v) = 2, marcando o vértice v com a cor 2.
        genes[v] = 2;

        // Obtém os vizinhos de v no grafo `h`.
        let neighbors: Vec<usize> = h
            .neighbors(&v)
            .map(|n| n.cloned().collect())
            .unwrap_or_default();

        // Passo 5: Se v tem vizinhos, escolha um (o primeiro da lista) e defina f(u) = 1.
        if let Some(first_neighbor) = neighbors.first() {
            genes[*first_neighbor] = 1;

            // Passo 6: Para os demais vizinhos de v, define f(w) = 0.
            for w in neighbors.iter().skip(1) {
                genes[*w] = 0;
            }
        }

        // Passo 7: Remove o vértice `v` e seus vizinhos do grafo `h`.
        let _ = h.remove_vertex(&v);
        for neighbor in neighbors {
            let _ = h.remove_vertex(&neighbor);
        }

        // Passo 8: Enquanto houver vértices isolados em h...
        let isolated_vertices = h.get_isolated_vertices();
        for z in isolated_vertices {
            genes[z] = 1;
            let has_neighbor_with_1 = graph
                .neighbors(&z)
                .map(|mut neighbors| neighbors.any(|n| genes[*n] == 1))
                .unwrap_or(false);

            // Verifica se `z` tem vizinhos no grafo original com f = 1.
            if !has_neighbor_with_1 {
                // Se não há vizinhos com f = 1, escolhe um vizinho com f = 0 e define f = 1.
                if let Some(mut neighbors) = graph.neighbors(&z) {
                    if let Some(first) = neighbors.find(|&n| genes[*n] == 0) {
                        genes[*first] = 1;
                    }
                }
            }

            // Passo 12: Remove o vértice `z` do grafo `h`.
            let _ = h.remove_vertex(&z);
        }
    }

    // Retorna a solução como um Chromosome, encapsulando o vetor de genes.
    Some(Chromosome::new(genes))
}

pub fn h3(graph: &UndirectedGraph<usize>) -> Option<Chromosome> {
    // Inicializa um vetor de genes com valores 0.
    // O tamanho do vetor é igual ao número de vértices no grafo.
    let mut genes = vec![0u8; graph.order()];

    // Faz uma cópia do grafo original para ser manipulado sem alterar o original.
    let mut h = graph.clone();

    // Enquanto o grafo h ainda tiver vértices... (Já captura o v = vértice de maior grau do grafo)
    while let Some(v) = h.vertices().max_by_key(|&vertex| h.degree(vertex)).cloned() {
        // Passo 4: Define f(v) = 2, marcando o vértice v com a cor 2.
        genes[v] = 2;

        // Obtém os vizinhos de v no grafo `h`.
        let mut neighbors: Vec<usize> = h
            .neighbors(&v)
            .map(|n| n.cloned().collect())
            .unwrap_or_default();

        // Ordena os vizinhos de forma decrescente pelo grau
        neighbors.sort_by(|&a, &b| h.degree(&b).cmp(&h.degree(&a)));

        // Passo 5: Se v tem vizinhos, escolha um (o primeiro da lista, ou seja, o com maior grau) e defina f(u) = 1.
        if let Some(first_neighbor) = neighbors.first() {
            genes[*first_neighbor] = 1;

            // Passo 6: Para os demais vizinhos de v, define f(w) = 0.
            for w in neighbors.iter().skip(1) {
                genes[*w] = 0;
            }
        }

        // Passo 7: Remove o vértice `v` e seus vizinhos do grafo `h`.
        let _ = h.remove_vertex(&v);
        for neighbor in neighbors {
            let _ = h.remove_vertex(&neighbor);
        }

        // Passo 8: Enquanto houver vértices isolados em h...
        let isolated_vertices = h.get_isolated_vertices();
        for z in isolated_vertices {
            // Caso contrário, define f(z) = 1.
            genes[z] = 1;
            let has_neighbor_with_1 = graph
                .neighbors(&z)
                .map(|mut neighbors| neighbors.any(|n| genes[*n] == 1))
                .unwrap_or(false);

            // Verifica se `z` tem vizinhos no grafo original com f = 1.
            if !has_neighbor_with_1 {
                // Se não há vizinhos com f = 1, escolhe um vizinho com f = 0 e define f = 1.
                if let Some(mut neighbors) = graph.neighbors(&z) {
                    if let Some(first) = neighbors.find(|&n| genes[*n] == 0) {
                        genes[*first] = 1;
                    }
                }
            }

            // Passo 12: Remove o vértice `z` do grafo `h`.
            let _ = h.remove_vertex(&z);
        }
    }

    // Retorna a solução como um Chromosome, encapsulando o vetor de genes.
    Some(Chromosome::new(genes))
}

// pub fn h2(graph: &UndirectedGraph<usize>) -> Option<Chromosome> {
//     let mut genes = vec![0u8; graph.order()];
//     let mut h = graph.clone();
//
//     // Passo 2: Enquanto tiver vértices em H faça (já capturo o vértice de maior grau):
//     while let Some(v) = h.vertices().max_by_key(|&v| h.degree(v)).cloned() {
//         genes[v] = 2;
//
//         // Obtém os vizinhos de v no grafo `h`.
//         let neighbors: Vec<usize> = h
//             .neighbors(&v)
//             .map(|n| n.cloned().collect())
//             .unwrap_or_default();
//
//         // Passo 5: Se v tem vizinhos, escolha um (o primeiro da lista) e defina f(u) = 1.
//         if let Some(first_neighbor) = neighbors.first() {
//             genes[*first_neighbor] = 1;
//
//             // Passo 6: Para os demais vizinhos de v, define f(w) = 0.
//             for w in neighbors.iter().skip(1) {
//                 genes[*w] = 0;
//             }
//         }
//
//         // Passo 7: Remove o vértice `v` e seus vizinhos do grafo `h`.
//         let _ = h.remove_vertex(&v);
//         for neighbor in neighbors {
//             let _ = h.remove_vertex(&neighbor);
//         }
//
//         // Passo 8: Enquanto houver vértices isolados em h...
//         let isolated_vertices = h.get_isolated_vertices();
//         for z in isolated_vertices {
//             // Verifica se `z` tem vizinhos no grafo original `graph` com f = 2.
//             let has_neighbor_with_2 = graph
//                 .neighbors(&z)
//                 .map(|mut neighbors| neighbors.any(|n| genes[*n] == 2))
//                 .unwrap_or(false);
//
//             // Se existe algum vizinho com f = 2, define f(z) = 0.
//             if has_neighbor_with_2 {
//                 genes[z] = 0;
//             } else {
//                 // Caso contrário, define f(z) = 1.
//                 genes[z] = 1;
//                 let has_neighbor_with_1 = graph
//                     .neighbors(&z)
//                     .map(|mut neighbors| neighbors.any(|n| genes[*n] == 1))
//                     .unwrap_or(false);
//
//                 // Verifica se `z` tem vizinhos no grafo original com f = 1.
//                 if !has_neighbor_with_1 {
//                     // Se não há vizinhos com f = 1, escolhe um vizinho com f = 0 e define f = 1.
//                     if let Some(mut neighbors) = graph.neighbors(&z) {
//                         if let Some(first) = neighbors.find(|&n| genes[*n] == 0) {
//                             genes[*first] = 1;
//                         }
//                     }
//                 }
//             }
//
//             // Passo 12: Remove o vértice `z` do grafo `h`.
//             let _ = h.remove_vertex(&z);
//         }
//     }
//
//     Some(Chromosome::new(genes))
// }

pub fn h5(graph: &UndirectedGraph<usize>) -> Option<Chromosome> {
    // Cria um vetor de genes com todos os vértices rotulados com valor 1;
    let genes: Vec<u8> = vec![1; graph.order()];
    Some(Chromosome::new(genes))
}
