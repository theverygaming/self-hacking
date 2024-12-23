import { getApiKey, getData, postData, putData, deleteData } from './api';

export interface QuestObj {
    name: string;
};

export interface QuestObjId extends QuestObj {
    id: string;
}

export interface QuestObjList {
    items: QuestObjId[];
    total_items: number;
}

export async function createQuest(data: QuestObj): Promise<QuestObjId> {
    return await postData("/quest/create", getApiKey(), data) as QuestObjId;
}

export async function getQuestsList(page: number, pageSize: number, search: string[][]=[]): Promise<QuestObjList> {
    const params = new URLSearchParams();
    params.append("page", (page-1).toString());
    params.append("pagesize", (pageSize).toString());
    params.append("search", JSON.stringify(search));

    const data = await getData(`/quest?${params.toString()}`, getApiKey()) as QuestObjList;
    return data;
}

export async function updateQuest(id: string, data: QuestObj) {
    await putData(`/quest/${id}`, getApiKey(), data);
}

export async function deleteQuest(id: string) {
    await deleteData(`/quest/${id}`, getApiKey());
}
