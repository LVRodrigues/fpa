# FPA Client

Design effort calculator using Function Point Analysis, based on information from the [International Function Point Users Group](https://ifpug.org).

## Uses

![GitHub](https://img.shields.io/github/license/LVRodrigues/fpa-management)

![Static Badge](https://img.shields.io/badge/angular-22.2-blue?logo=angular)
![Static Badge](https://img.shields.io/badge/SAAS-yellow)
![Static Badge](https://img.shields.io/badge/NGXecharts-yellow)
![Static Badge](https://img.shields.io/badge/RSA-yellow)

## Desenvolvimento

O cliente usa Angular/CLI/Material 22.2 e TypeScript 6.0. Use Node.js
`^22.22.3 || ^24.15.0 || >=26.0.0`, conforme os requisitos do Angular.

```bash
npm ci
npm start
```

`npm run pt` inicia a interface em português. `npm run build` gera os dois
idiomas (`en` e `pt`) em `dist/fpa-client/browser/`.

## Publicar Nova Versão

Using the [GitFlow](https://www.atlassian.com/git/tutorials/comparing-workflows/gitflow-workflow) workflow, run the commands:

```bash
git flow release start <id>
npm run release-patch
git commit -a -m "Versão ???"
git flow release finish
```

The version is reported by three fields: [major, minor, patch]. The above command increments only the last field. To increment the minor field, use **npm run release**.