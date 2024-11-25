import { getData, postData } from './api';

export interface BrainlogObj {
    body: string;
    log_type: string;
    time: string;
};

export interface BrainlogObjId extends BrainlogObj {
    id: string;
}

function getApiKey(): string {
    const api_key = localStorage.getItem("API_KEY");
    console.log(api_key);
    if (!api_key) {
        throw new Error("missing API key");
    }
    return api_key;
}

export async function getType(id: string) {
    return await getData(`/brainlog/type/get?id=${id}`, getApiKey());
}

export async function createLog(data: BrainlogObj): Promise<BrainlogObjId> {
    return await postData("/brainlog/create", getApiKey(), data) as BrainlogObjId;
}

export async function getLogsList(page: number, pageSize: number): Promise<BrainlogObjId[]> {
    let data = await getData(`/brainlog/list?page=${page-1}&pagesize=${pageSize}`, getApiKey()) as BrainlogObjId[];
    return data;
}

export async function updateLog(id: string, data: BrainlogObj) {
    await postData(`/brainlog/update?id=${id}`, getApiKey(), data);
}

export async function deleteLog(id: string) {
    await getData(`/brainlog/delete?id=${id}`, getApiKey());
}
