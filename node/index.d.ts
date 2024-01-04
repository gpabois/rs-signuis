export type Result<D, E> = D | E;

export interface Issue {
    code: string,
    message: string,
    path: Array<string> 
}

export interface Error {
    code: string,
    message: string,
    issues: Array<Issue>
}

declare function report_nuisance(report: CreateNuisanceReport): Promise<Result<NuisanceReport, Error>>;