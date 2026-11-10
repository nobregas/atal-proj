pub fn movimentar() {
    // recebe Entrada
    // somente move direita [linha][coluna+1] ou baixo [linha+1][coluna]
    // nao move se obstaculo custo = None
    // logo se direita = None e baixo = None, retorna "Sem caminho"
}


pub fn resolver_recursivo() {
    //backtrack
    // decisao 1 - mover direita ou esquerda (se nao obstaculo)
    // registra movimento escolhido 
    // chama recursivamente com nova posicao
    // se chegar a posicao travada ou a posicao final, retorna caminho escolhido
    // ao final deve retornar o caminho escolhido e o custo total
    // e numero de movimentos realizados
}

pub fn resolver_guloso() {
    // pega posicao atual linha atual e coluna atual
    // obtem os valores das bordas [linha][coluna+1] e [linha+1][coluna]
    // se custo = None, retorna "Sem caminho"
    // movimentar() para a borda com menor custo
    // repete ate chegar na posicao final
    // ao final deve retornar o caminho escolhido e o custo total
    // e numero de movimentos realizados
}