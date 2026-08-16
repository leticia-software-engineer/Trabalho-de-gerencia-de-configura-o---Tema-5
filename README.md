# Integração,  Entrega e Implantação Contínua(CI/CD)

#### Como as práticas de Integração Contínua, Entrega Contínua e Implantação Contínua automatizam o ciclo de desenvolvimento e disponibilização de software.

---

## 1. Introdução

Atualmente , muito se fala no mundo do desenvolvimento de software, sobre Integração Contínua, Implantação e Entrega Contínua e todo o ciclo que gira em torno do CI/CD. Entender tais conceitos, bem como suas aplicações, se tornou um conhecimento essencial para trabalhar nessa área. Porém, para além de entender é necessário saber aplicar, saber diferenciar, saber criticar e ter um posicionamento crítico, bem como estar por dentro de tudo que permeia essas novas tecnologias.

Pensando nisso, a presente pesquisa busca contextualizar de maneira didática o conhecimento que desenvolvemos acerca das ferramentas e plataformas de CI/CD, suas principais arquiteturas, vantagens e desvantagens, funcionalidades , estratégias de implantação, mecanismos de aprovação, Rollback, monitoramento pós-deploy, cenários de utilização e por último, mas não menos importante, o nosso exemplo prático de um pipeline, exemplificando o funcionamento real e permitindo que você, leitor, experimente também a maneira prática de aplicar esses conceitos.

Para além de uma simples revisão bibliográfica, na presente pesquisa buscamos promover discussões, diferentes perspectivas e abordagens voltadas ao CI/CD e integrar teoria e prática num estudo mais experimental.

---
## 2. Metodologia

O propósito desta pesquisa consiste em fazer uma análise crítica dos conceitos, mecanismos de implantação, plataformas e arquiteturas de CI/CD. Para sua elaboração foi realizada uma pesquisa documental, de caráter exploratório e abordagem mista, mediante a utilização de dados estatísticos, bem como interpretativos, com um desenho transversal no que diz respeito ao tempo. O que buscamos analisar não apenas o que é o CI/CD e discutir o que já se sabe, mas discutir o porquê do que se sabe, discutir vantagens, desvantagens e as implicações das decisões de projeto quando se usa e quando não se usa essas práticas.

---
## 3. Fundamentação teórica

### 3.1. Integração Contínua

A Integração contínua, é a prática de mesclar todas as cópias de trabalho dos desenvolvedores em uma linha principal compartilhada, várias vezes ao dia.[1] Proposta pela primeira vez em 1991, por Grady Booch, inicialmente não definia a necessidade de integração várias vezes ao dia, contudo com o surgimento do Extreming Programming, essa prática passou a ser defendida com mais frequência. Dessa maneira, é incontestável que no contexto atual, muitas coisas mudaram em relação a data de surgimento dessa prática, porém, cada vez mais ela se fez necessária no cotidiano de quem trabalha com desenvolvimento de software. 
 
Assim, integração contínua hoje não é só uma boa prática, mas um pré requisito para um bom desenvolvimento. Isso se dá ao fato de que o desenvolvimento é contínuo, os desenvolvedores trabalham em equipes, muitas vezes até separadas fisicamente, os projetos são longos e as entregas precisam ser frequentes para manter o ritmo ágil exigido pelas empresas e pelos clientes e a maneira mais ágil de integrar esse código é por meio do CI. Com o CI, são feitos muitos commits ao longo do dia e o merge é feito com frequência evitando o máximo de conflitos.

---
### 3.2. Entrega contínua

A Entrega contínua, por sua vez, aborda todo o fluxo de desenvolvimento do projeto de software, visando simplificar o lançamento de entregas de ponta-a-ponta, por meio de feedbacks mais curtos entre os desenvolvedores e seus stakeholders. Estes feedbacks são realizados desde o desenvolvimento do código até a implantação / produção. [2] Tais lançamentos contínuos auxiliam em um melhor suporte e identificação de pontos de melhorias no software, além de minimizar maiores riscos ao identificar os erros precocemente, para serem corrigidos e atualizados em novas versões do sistema, seja em qualquer etapa do projeto.


As etapas de entrega contínua são definidas para promover um ambiente contínuo de homologação e planejamento, para que as entregas sejam devidamente testadas e implementadas da maneira correta. Sendo esses:

- 1.  Configuração do deploy: Criação de ambientes idênticos de teste, homologação e produção, além de ferramentas de automação utilizadas no processo;
     
- 2. Planejamento: Etapa em que são definidas as tarefas à serem desenvolvidas e executadas durante uma sprint;
     
- 3. Execução: Etapa em que tem como objetivo a qualidade do produto que está sendo desenvolvido, no qual é implementados testes contínuos a fim de evitar problemas nas próximas etapas, utilizando ferramentas de verificação  da qualidade;

- 4. Inspeção e Adaptação: Etapa em que executa os testes automatizados, relacionados à Integração Contínua (CI), validando se a entrega atende os requisitos necessários para seguir à próxima etapa;
     
- 5. Operação e Suporte: Nesta etapa, são realizadas implantações automatizadas das entregas, por meio de um ambiente de homologação para testes. Posteriormente, caso tudo estiver nos conformes, é seguido para o ambiente de produção. Caso houver erros no desempenho da aplicação dão resolvidos em uma próxima sprint, ou na sprint atual, a depender da gravidade do erro apresentado.
 
     
---
### 3.3. Implantação contínua

A implantação contínua (CD) é um processo de lançamento de software que usa testes automatizados para validar se as alterações em uma base de código estão corretas e estáveis para implantação autônoma imediata em um ambiente de produção.[3] De um modo geral, trata-se de uma implantação que considera os testes como uma validação automática daquilo que foi produzido e automatiza o processo de deploy, agilizando essa etapa do desenvolvimento. Isso é extremamente útil, quando há um grande volume de alterações em um projeto ou o sistema é muito grande e contém várias entregas frequentes, economizando tempo, trabalho e mão de obra, já que o processo é automático.

---
### 3.4. Diferenciação

Integração contínua, entrega e implantação contínua não é a mesma coisa, mas se complementam. Mas qual a diferença entre elas e porque juntas são tão poderosas?

A diferença principal entre a entrega contínua e a implantação contínua é que a entrega é focada em preparar o código para a implantação, mas deve aguardar algum tipo de aprovação humana antes de ser implementada, enquanto a implantação, automatiza completamente esse processo, assim que uma mudança passa pelos testes, ela é implantada automaticamente.

Por outro lado, a integração contínua é focada em juntar todo o projeto, unindo alterações feitas por todos os membros da equipe diariamente com a maior frequência possível de modo a manter o sistema sempre sincronizado e atualizado por todos e para todos, agilizando o processo posterior da entrega contínua. Então o CI é um acelerador do CD, é um processo que ao ser integrado ao CD, agiliza todo o desenvolvimento do software e eles se complementam como boas práticas.

Logo, vemos que o CI e o CD são práticas modernas e muito requisitadas para quem deseja trabalhar na área de TI e por isso neste estudo buscamos aprofundar uma discussão a respeito dessas práticas de modo a consolidar o seu entendimento acerca desse tema.

---
## 4. Ferramentas e plataformas de CI/CD

Depois de escolher utilizar CI/CD é preciso saber como utilizar e para isso é necessário ter o conhecimento das ferramentas que dão suporte a essas práticas e é dessas ferramentas que vamos falar agora, fazendo uma análise comparativa do que está disponível, quais as vantagens e desvantagens dessas ferramentas e o que é qual a melhor ferramenta para o seu projeto.

---

### 4.1. Análise comparativa

- a. **Requisitos de Visão Plataforma ALM:** Requisitos, testes, riscos, defeitos, arquitetura, linhas de base e evidências de auditoria estão todos interligados em um único sistema controlado. Essa ferramenta é útil para quando se deseja acompanhar o processo de ponta a ponta considerando toda a engenharia do software, principalmente a rastreabilidade dos requisitos. Essa ferramenta possui alta incorporação de IA para acompanhamento dos processos. As principais vantagens estão relacionadas ao controle e rastreabilidade facilitando auditoria, sendo interessante para projetos que envolvem, por exemplo, verbas governamentais e necessitam de auditoria frequente. Enquanto algumas desvantagens são o custo elevado , pensando no orçamento e a implementação é lenta pois necessita de muita especificação. 

- b.**Jenkins**: O Jenkins funciona como um servidor de automação de integração contínua e entrega contínua (CI/CD), ajudando os desenvolvedores a criar, testar e implantar aplicativos com eficiência. Em sua essência, o Jenkins segue uma abordagem orientada a pipelines.[4] O Jenkins é uma das ferramentas de CI/CD mais utilizadas por possuir alta flexibilidade e escalabilidade, possui alto ecossistema de plugins e uma vasta comunidade de desenvolvedores que o utilizam, facilitando a resolução de problemas. Entretanto, a sua configuração exige um alto custo de manutenção, em aplicações de larga escala ele não é muito recomendado, pois podem surgir gargalos de desempenho.

- c. **CI / CD do GitLab**: O GitLab CI é usado para automatizar e acelerar as etapas de teste, construção, implantação e entrega de sistemas. Enquanto o GitLab CD permite que desenvolvedores de software automatizem o processo de implantação de seus aplicativos em ambientes diversos. Algumas vantagens dessa ferramenta são, por exemplo, o CI baseado em chat, vinculação do Docker, conexão com repositórios externos, runners CI dimensionáveis automaticamente. Mas ao mesmo tempo vários desses recursos só estão disponíveis na versão premium e possuem um custo financeiro associado, e muitas vezes é percebida uma certa dificuldade para migrar um projeto da versão gratuita para a premium. Mas, considerando projetos de grande escala, essa ferramenta é mais vantajosa do que o jenkins e costuma apresentar alta confiabilidade.

- d. **GitHub Actions**: É uma ferramenta que permite automatizar, personalizar e executar fluxos de trabalho de desenvolvimento de software diretamente no repositório do Github. Ele pode ser usado para implementar tanto CI quanto CD. O GitHub fornece máquinas virtuais do Linux, Windows e macOS para a execução dos seus fluxos de trabalho e possui como componentes principais os fluxos de trabalho (workflows), eventos, trabalhos e ações.[5] As principais vantagens dessa ferramenta são voltadas a simplicidade pois ela é integrada ao Github que já é muito utilizado pelos desenvolvedores para armazenamento de código, também oferece suporte a diversos sistemas operacionais e linguagens de programação e possui uma comunidade vasta de desenvolvedores. E uma das vantagens dele que se sobrepõe às ferramentas mencionadas anteriormente é justamente o potencial do plano gratuito que permite uma ampla gama de recursos para projetos open source.


- e. **Pipelines Bitbucket**: o Bitbucket é um sistema de versionamento distribuído baseado no Git. Possui integração total com outros produtos da Atlassian, como Jira, Fisheye e Bamboo. Uma das maiores vantagens que a Bitbucket tem sobre seus concorrentes é oferecer um número ilimitado de repositórios privados. Pipelines Bitbucket é um serviço integrado de CI/CD embutido na Nuvem Bitbucket. Então sabemos que o bitbucket funciona como o github porém possui suas peculiaridades. Mas como os outros também possui suas desvantagens, uma desvantagem clara é voltada a limitação com suporte apenas a redes IPV4, além dos minutos de build serem limitados, então para projetos muitos grandes com build longos o bitbucket pipelines não é a melhor opção. 

- f. **AWS Code Pipeline**: Essa ferramenta é integrada com a plataforma de armazenamento em uvem da AWS e permite que o processo de CI e CD seja feito nesse mesmo ambiente possui armazenamento de artefatos baseado em S3 e a infraestrutura é gerenciada pela própria AWS, sendo uma boa escolha para projetos grandes ou que já utilizam a AWS. Algumas desvantagens dessa ferramenta são principalmente voltadas a interface ser pouco intuitiva e alguns problemas de usabilidade como a latência e também o forte acoplamento com a AWS, o que dificulta bastante uma migração para outra plataforma.

---

## 5. Arquiteturas 

A Arquitetura de CI/CD refere-se à organização estrutural e operacional dos sistemas que automatizam a compilação, teste, empacotamento e implantação de software. Ela define como o código flui do repositório até o ambiente final de produção, impactando diretamente a segurança, escalabilidade, tempo de execução e a superfície de ataque da infraestrutura. Com o passar do tempo, as arquiteturas evoluíram de servidores simples para modelos mais avançados, distribuídos e nativos da nuvem.

### 5.1. Modelos arquiteturais de CI/CD:

Arquitetura Push-Based: O servidor de CI/CD (ou runner) escuta eventos do repositório (via webhooks), executa o build, roda os testes e se conecta diretamente ao ambiente de destino para aplicar as alterações.
> - Fluxo: Git Commit ➔ Servidor CI/CD ➔ Autenticação Externa ➔ Push para Produção
> - Exemplos: GitHub Actions, GitLab CI, Jenkins, CircleCI, AWS CodePipeline.
> - Vantagens: Configuração inicial rápida, suporta múltiplos alvos (SaaS, VMs, Bare Metal), visibilidade centralizada do pipeline
> - Desvantagens: Exige expor portas ou credenciais de produção para o CI, risco de gargalo se os runners forem compartilhados
> - Caso de uso ideal: Projetos heterogêneos, monorepos, arquiteturas baseadas em VMs e funções Serverless. 

Arquitetura Pull-Based (GitOps): Nenhum sistema externo possui credenciais de acesso ao ambiente final. Em vez disso, um agente ou operador interno (instalado dentro do cluster de destino) monitora continuamente o repositório Git e puxa (pull) o estado desejado, aplicando-o localmente.
> - Fluxo: Git Commit ➔ Agente Interno (In-Cluster) ➔ Sincronização Local
> - Exemplos: Argo CD, Flux CD, Fleet.
> - Vantagens: Sem credenciais expostas para fora, detecção automática de drift (desvio de configuração), rollback trivial apontando para um commit anterior
> - Desvantagens: Exige que a infraestrutura seja 100% declarativa, curva de aprendizado alta, restrito principalmente a contêineres e Kubernetes
> - Caso de uso ideal: Ambientes nativos em Kubernetes, arquiteturas de microsserviços altamente reguladas. 

Arquitetura Efêmera e Serverless (Kubernetes-Native): Não existem executores (runners) fixos ligados o tempo todo. Cada etapa do pipeline dispara a criação de um pod ou contêiner temporário isolado em um cluster, que é destruído imediatamente após a conclusão.
> - Fluxo: orientado a eventos e o ciclo de vida da própria infraestrutura faz parte do fluxo. O pipeline não se conecta a um servidor existente, ele cria o servidor, executa o trabalho e destrói o servidor. 
> - Exemplos: Tekton Pipelines, Jenkins X.
> - Vantagens: Isolamento total entre jobs (sem estado residual), uso eficiente de recursos (paga/aloca apenas quando executa), escalabilidade horizontal ilimitada
> - Desvantagens: Cold start (atraso para subir imagens do runner), gestão complexa de cache entre execuções
> - Caso de uso ideal: Grandes organizações com centenas de deploys diários e necessidade de isolamento estrito.

---

## 6. Funcionalidades

É comum, quando se trata de funcionalidades, que elas sejam implementadas para todos os usuários. Com os recursos de sinalização (feature flags), a implantação (deploy) do código ocorre separadamente da liberação (release) para os usuários, sem estar imediatamente visível ou ativo para todos os usuários. Dessa forma, os desenvolvedores podem testar funcionalidades em um ambiente de produção e, somente após a confirmação de que tudo está funcionando corretamente, a funcionalidade pode ser ativada para os usuários. Esse desacoplamento permite implantações mais seguras, menores e mais frequentes (reduzindo o risco de incidentes graves), além de funcionar como um botão de desativação, caso ocorra um erro em produção. 

As features flags funcionam como uma condição lógica que determina se um determinado trecho de código será executado ou não. Elas são divididas em 4 tipos principais: flags de release (usadas para controlar o lançamento de funcionalidades), flags de experimento (utilizadas em testes de comparação de comportamentos ou interfaces), flags operacionais (servem para ativar e desativar recursos em situações urgentes) e flags de permissão (definem o acesso de funcionalidades de acordo com perfis de usuário)

### 6.1. Estratégias de liberação:

- Trunk-Based Development: Os desenvolvedores integram suas alterações diretamente na branch principal múltiplas vezes ao dia. Nessa estratégia, os recursos incompletos ficam escondidos atrás de uma flag.
- Canary Releases Nível de Aplicação: Diferente do Canary no nível de infraestrutura (roteamento de tráfego de rede), a flag direciona percentuais de usuários na própria camada de software com base em regras de contexto (ex: ID do usuário, e-mail, grupo).
- Rollback Instantâneo: Se uma falha for detectada em produção, a funcionalidade é desligada alterando uma variável na ferramenta de Feature Management. Não é necessário fazer um novo deploy ou build nem reverter alterações no Git.

---

## 7. Estratégias de implantação

As Estratégias de Implantação definem a metodologia pela qual uma nova versão de um software substitui a versão antiga em um ambiente de produção. O principal objetivo de analisar e escolher a melhor estratégia é minimizar ou até mesmo eliminar o tempo de inatividade (downtime), reduzir o risco de falhas e permitir uma reversão rápida (rollback) caso ocorra algum problema. Diferente do Feature Management (que ocorre na camada lógica da aplicação), as estratégias de implantação operam na camada de infraestrutura e roteamento de rede (Load Balancers, Ingress Controllers, DNS, Orquestradores de Contêineres).

### 7.1. Principais Estratégias de Implantação

- Recreate (Recriação): É uma abordagem mais simples e bruta. Nela todas as instâncias da versão antiga (V1) são destruídas simultaneamente antes que as instâncias da nova versão (V2) sejam provisionadas.
Mecanismo: Derruba V1 ➔ Espera ➔ Sobe V2
Caso de Uso: Em aplicações legadas, ambientes de desenvolvimento ou quando há alterações estruturais no banco de dados que não suportam duas versões rodando ao mesmo tempo.
- Rolling Update (Atualização Progressiva / Ramped): Esse modelo propõe a substituição das instâncias antigas por novas de forma gradual, uma a uma ou em pequenos lotes. É o padrão nativo do Kubernetes (Deployment).
Mecanismo: Sobe 1x V2 ➔ Valida ➔ Derruba 1x V1 ➔ Repete até 100%
Caso de Uso: Em aplicações stateless (sem estado) onde a V1 e a V2 são totalmente compatíveis e podem coexistir acessando o mesmo banco de dados temporariamente.
- Blue-Green (Red-Black): Mantém dois ambientes de infraestrutura idênticos. O ambiente Blue hospeda a versão atual de produção. O ambiente Green recebe o deploy da nova versão. Os testes são feitos no Green, se aprovados, o balanceador de carga redireciona 100% do tráfego do Blue para o Green instantaneamente.
Mecanismo: Deploy no Green ➔ Testes em Produção Invisível ➔ Troca o Roteamento (Switch)
Caso de Uso: Em sistemas críticos que não toleram downtime e precisam de um rollback imediato (basta voltar o tráfego para o Blue).
- Canary Deployment (Canário): A nova versão (V2) é liberada para uma pequena porcentagem do tráfego real. Se as métricas de erro (logs, latência, health checks) permanecerem estáveis, o tráfego é ampliado gradativamente (10%, 25%, 50%, 100%).
Mecanismo: V2 recebe 5% do tráfego ➔ Análise de Métricas ➔ Escala para 100%
Caso de Uso: Para grandes plataformas B2C (Netflix, Amazon, Google) onde testes sintéticos não cobrem todos os comportamentos imprevisíveis dos usuários.
- Shadow Deployment (Dark Launching): O tráfego real dos usuários que chega na versão antiga (V1) é espelhado (duplicado) para a nova versão (V2) nos bastidores. A V2 processa as requisições, mas suas respostas são descartadas. O usuário só recebe a resposta da V1.
Mecanismo: Tráfego na V1 ➔ Fork assíncrono para V2 ➔ Comparação de resultados e performance
Caso de Uso: Refatorações pesadas (ex: troca de linguagem de programação, migração de banco de dados) onde você precisa provar que a nova versão suporta a carga e gera os mesmos resultados.

---

## 8. Mecanismos de aprovação

---

## 9. Rollback

Quando é identificado um bug, problemas de compatibilidade ou interrompimento de um processo crucial no software em desenvolvimento, retornar para a versão mais estável das sprints anteriores pode auxiliar na identificação e correção do bug apresentado, revertendo para o estado do projeto já conhecido pela equipe. [9] Por isso, o _**Rollback**_ é um dos mecanismos implementados no desenvolvimento contínuo de software, permitindo uma válvula de escape para evitar transtornos maiores e/ou riscos à segurança e dados do sistema. 

O _Rollback_ se torna um recurso que oferece aos desenvolvedores uma maneira segura de retornar o projeto para um _backup_ anterior, em casos de erros identificados na _sprint_ mais recente, para que assim seja possível a identificação e correção da funcionalidade com algum tipo de erro ou risco. Porém, em casos de riscos que podem se tratar de pontos mais críticos do sistema, é importante verificar outra medida como alternativa para correção, sendo o _rollback_ a última alternativa para tais ocorrências.

---

### 9.1 Blue-Green Deployment

Uma das maneiras de controle de versões e updates que melhor permitem o uso do rollback é a Implantação Azul-Verde (**_Blue-Green Deployment_**) [11], que é um modelo de lançamento de aplicações que permite uma transição segura do tráfego de usuários entre versões, sem o risco de queda do sistema, por meio de ambientes de produção. 

Nesta técnica, o ambiente atual com a versão antiga será denominada com uma _flag_ azul, enquanto que a nova versão será implementada em um ambiente com a _flag_ verde. Caso a transição seja bem sucedida, o ambiente azul pode ser retirado da produção, sendo o ambiente verde o novo azul, mas caso precisar, é possível realizar o _rollback_ para a versão antiga, sem risco de queda do software que está sendo usado na ocasião.

---

## 10. Monitoramento pós-deploy

Para além das implantações e desenvolvimento contínuo proposto pela pipeline CI/CD, também se faz necessário acompanhar e monitorar as métricas de desempenho e produção do software, assim como a saúde do sistema e do processo em andamento da equipe de desenvolvimento, para que assim não se reflita no produto final possíveis atrasos e dívidas técnicas.

Conforme definido no artigo publicado pelo portal Splunk [12], é necessário as equipes monitorarem as seguintes métricas no fluxo de trabalho de CI/CD, garantindo uma rápida identificação de sinais críticos e oportunidades de otimização:

- Frequência / Tempo de implantação;
- Prazo de Execução de Alterações;
- Tempo médio para recuperar / resolver um erro de produção;
- Taxa de falha em alterações;
- Tempo de espera de alterações do sistema na pipeline.

Tais métricas são monitoradas com o auxílio de ambientes e dashboards apresentados anteriormente (Jenkins, Github Actions, Gitlab CI), podendo ser analisado e discutido pela equipe como um todo.

---

## 11. Cenários de utilização

Com isso, a pipeline de entrega e implantação contínua é uma ferramenta importante para o gerenciamento da equipe no desenvolvimento de sistemas voláteis, que necessitam de constantes atualizações e feedbacks, além de efetuar testes automatizados complexos e implantações no ambiente de nuvem, com a possibilidade de escalonamento e alto tráfego de usuários. 

Os principais cenários de uso do ecossistema abrangido são:

- Aplicações Cloud Native;
- Aplicações Web;
- Aplicações SaaS (Software as a Service);
- Microsserviços;
- Aplicativos Móveis.

Esses softwares exigem constantes testes e atualizações em seu fluxo de desenvolvimento, por lidarem com arquiteturas complexas e alto tráfego de dados e usuários. Dessa forma, o pipeline CI/CD se torna uma ferramenta útil para tais casos de uso.

---

## 12. Nosso exemplo prático

13


- 13. Conclusão

14

---

### Equipe
---
Gabriel Souza Santos

Herberthy Samir Oliveira F. de Souza

Joaquim Arthur Muniz Leite

José Dhonatan Fernandes de Almeida

José Welton de Sousa Melo 
- Github: @jweltu
- Lattes: http://lattes.cnpq.br/4907634044510168

Letícia Maria dos Santos Dias

Sarah Mendes Teles

---
## 14. Referências

[1] WIKIPEDIA CONTRIBUTORS. Integração contínua. Disponível em: <https://pt.wikipedia.org/w/index.php?title=Integra%C3%A7%C3%A3o_cont%C3%A Dnua&oldid=60895845>. [URL 🔗](https://pt.wikipedia.org/w/index.php?title=Integra%C3%A7%C3%A3o_cont%C3%ADnua&oldid=60895845)

[2] DE SOUSA, Sabryna et al. Entrega Contínua de Software: Um estudo de caso. [S.l.: S.n.]. Disponível em: <https://encurtador.com.br/mOYO>. Acesso em: 13 de agosto, 2026. [URL 🔗](https://encurtador.com.br/mOYO)

[3] ATLASSIAN. O que é implantação contínua? Disponível em: <https://www.atlassian.com/br/continuous-delivery/software-testing/continuous-deplo yment>. Acesso em: 13 ago. 2026. [URL 🔗](https://www.atlassian.com/br/continuous-delivery/software-testing/continuous-deployment)

[4] What is Jenkins? A Guide to CI/CD. Cloudbees.comCloudBees, , 10 dez. 2025. Disponível em: <https://www.cloudbees.com/blog/what-is-jenkins>. Acesso em: 15 ago. 2026 [URL 🔗](https://www.cloudbees.com/blog/what-is-jenkins)

[5] GitHub Actions: o que é, como funciona e boas práticas! Disponível em: <https://focusnfe.com.br/blog/github-actions/>. Acesso em: 15 ago. 2026. [URL 🔗](https://focusnfe.com.br/blog/github-actions/)

[6] DEVOPS. Red Hat, [s.d.]. Disponível em: https://www.redhat.com/pt-br/topics/devops. Acesso em: 15 ago. 2026. 

[7] CI/CD vs DevOps | Implementation Best Practices. LaunchDarkly, 31 maio 2024. Disponível em: https://launchdarkly.com/blog/devops-vs-cicd/. Acesso em: 15 ago. 2026. 

[8] SOUZA, Gabriel. Feature Flags: Conceitos, Benefícios e Boas Práticas. Medium, 17 jan. 2026. Disponível em: https://medium.com/@gabweb95/feature-flag-conceitos-benef%C3%ADcios-e-boas-pr%C3%A1ticas-51461c1d8302. Acesso em: 15 ago. 2026. 

[9] IBSEC, Equipe. Rollback: O que é e como funciona no Git. IBSEC, 7 ago. 2025. Disponível em: <https://ibsec.com.br/rollback-o-que-e-e-como-funciona-no-git/>. Acesso em: 15 ago. 2026 

[10] Harness. Understanding rollbacks in software development. Disponível em: <https://www.harness.io/blog/understanding-software-rollbacks>. Acesso em: 15 ago. 2026. 

[11] Red Hat. O que é implantação azul-verde? Disponível em: <https://www.redhat.com/pt-br/topics/devops/what-is-blue-green-deployment>. Acesso em: 15 ago. 2026. 

