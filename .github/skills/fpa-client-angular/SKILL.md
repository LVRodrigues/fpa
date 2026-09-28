---
name: fpa-client-angular
description: 'Desenvolver ou manter o cliente Angular do FPA Management: telas de projetos, fronteiras e funções, análise por pontos de função, integração com a API fpa-server, autenticação OIDC, formulários, navegação e localização. Use ao implementar, depurar ou validar fluxos do fpa-client.'
argument-hint: 'Descreva a tela, o fluxo ou a integração do cliente FPA que deseja alterar.'
---

# Cliente Angular FPA

Ajude a implementar e validar fluxos do `fpa-client` para gestão e análise de projetos por pontos de função, mantendo os contratos com o `fpa-server` e as convenções locais.

## Quando usar

- Criar ou alterar telas, formulários, navegação e estados de projetos, fronteiras e funções.
- Integrar o cliente Angular com endpoints do `fpa-server`.
- Corrigir problemas de autenticação, apresentação de dados, localização ou testes no cliente.

## Procedimento

1. **Delimite o fluxo.** Identifique o resultado esperado, os componentes/rotas envolvidos e o caminho do usuário. Inspecione primeiro os arquivos próximos em `fpa-client/src/app/`; preserve a estrutura existente e evite mudanças não relacionadas.

2. **Confirme o contrato de domínio.** Para qualquer chamada ao servidor, confira as rotas, parâmetros, respostas e erros em `fpa-server/src/handlers/`, `fpa-server/src/docs.rs`, modelos e testes pertinentes. Não deduza nomes de campos, paginação, operações ou regras de análise apenas pela interface. Se o contrato não resolver uma ambiguidade funcional, pergunte antes de codificar essa parte.

3. **Respeite o modelo de pontos de função.** O servidor distingue funções de dados (`ALI`, `AIE`) e funções transacionais (`EE`, `CE`, `SE`). As funções de dados contêm RLRs, que contêm DERs; as transações referenciam funções de dados por ALRs. Fatores e empíricos são associados à fronteira do projeto. Preserve esses tipos e associações no cliente e confirme seus nomes/serialização no contrato atual. Essa taxonomia não define, por si só, regras de classificação, matrizes de complexidade, pesos ou totais: não os invente nem os calcule com constantes presumidas. Para implementar esses cálculos, exija uma regra aprovada e localizável no servidor, testes, documentação do projeto ou requisito explícito do usuário.

4. **Siga a arquitetura do cliente.** O projeto usa Angular 19, componentes standalone e Angular Material; as rotas ficam em `src/app/app.routes.ts`. Verifique os componentes e serviços vizinhos antes de escolher onde colocar estado e lógica. Não adicione bibliotecas nem abstrações sem uma necessidade demonstrável.

5. **Integre com segurança.** O cliente configura OIDC em `src/app/auth.config.ts` e registra o cliente HTTP com o interceptor de autenticação em `src/app/app.config.ts`. Reutilize essa configuração; não armazene nem envie tokens manualmente. Antes de definir uma URL base, confira `src/environments/`, a configuração do servidor e os ambientes de execução. Garanta que rotas protegidas e chamadas autenticadas sigam o comportamento existente.

6. **Complete o fluxo da interface.** Considere estados de carregamento, vazio, sucesso e erro, validação de campos e retorno claro após operações. Mantenha a navegação e os breadcrumbs coerentes com as etapas existentes. Para texto visível ao usuário, confira o processo de localização e os arquivos em `src/locale/`.

7. **Valide a mudança.** Execute primeiro o teste mais específico disponível. Para alterações no cliente, use os scripts de `fpa-client/package.json`, como `npm test` e `npm run build`, conforme o risco e a configuração afetada. O projeto usa Docker Compose, ativo por padrão, com os serviços `oauth-2` e `fpa-server` disponíveis; reutilize-os ao validar o fluxo integrado e só os inicie se estiverem parados. Informe comandos executados, resultado e qualquer verificação que não pôde ser feita.

8. **Feche o escopo.** Revise as alterações para confirmar que contrato, domínio, autenticação, navegação e textos estão consistentes. Resuma o comportamento entregue e liste dúvidas ou limitações ainda abertas; não declare uma integração end-to-end validada sem exercitá-la.

## Critérios de qualidade

- Tipos e payloads do cliente correspondem ao contrato atual do servidor.
- A interface cobre erros, dados vazios e carregamento sem ocultar falhas da API.
- Autenticação, rotas e localização permanecem compatíveis com a configuração existente.
- Mudanças ficam concentradas no fluxo solicitado e passam pelas verificações disponíveis.
- Sistema utiliza internacionalização, com o padrão inicial em Inglês e tradução para Português do Brasil.