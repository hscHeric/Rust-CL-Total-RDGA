import sys
import networkx as nx

def carregar_lista_adjacencia(arquivo):
    """Carrega a lista de adjacência de um arquivo .txt e cria um grafo NetworkX."""
    grafo = nx.Graph()
    with open(arquivo, 'r') as f:
        for linha in f:
            elementos = linha.strip().split()
            if elementos:
                vertice = elementos[0]
                adjacentes = elementos[1:]
                for adj in adjacentes:
                    grafo.add_edge(vertice, adj)
    return grafo

def remover_vertices_isolados(grafo):
    """Remove os vértices isolados de um grafo NetworkX."""
    isolados = list(nx.isolates(grafo))
    grafo.remove_nodes_from(isolados)
    return grafo

def salvar_lista_adjacencia(grafo, arquivo):
    """Salva a lista de adjacência de um grafo NetworkX em um arquivo .txt."""
    with open(arquivo, 'w') as f:
        for vertice in grafo.nodes:
            adjacentes = list(grafo.adj[vertice])
            f.write(f"{vertice} {' '.join(adjacentes)}\n")

def main():
    if len(sys.argv) != 2:
        print("Uso: python script.py <arquivo_de_entrada>.txt")
        sys.exit(1)

    entrada = sys.argv[1]
    saida = entrada.replace('.txt', '_normalized.txt')

    # Carregar grafo
    grafo = carregar_lista_adjacencia(entrada)

    # Remover vértices isolados
    grafo_normalizado = remover_vertices_isolados(grafo)

    # Salvar grafo normalizado
    salvar_lista_adjacencia(grafo_normalizado, saida)

    print(f"Lista de adjacência normalizada salva em '{saida}'.")

if __name__ == "__main__":
    main()

