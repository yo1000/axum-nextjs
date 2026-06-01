'use client';

import ApiClient from "@/utils/ApiClient";
import {PagedData} from "@/components/Table";

export type ItemInventory = {
    id: number;
    item: any;
    quantity: number;
};

export default class item_inventoriesApiClient {
    private readonly apiClient: ApiClient
    private readonly baseUri: string

    constructor(accessToken: string | undefined, baseUri: string | undefined) {
        this.apiClient = new ApiClient(accessToken);
        this.baseUri = baseUri ?? ``;
    }

    public async get(page?: number): Promise<PagedData<ItemInventory> | undefined> {
        return await this.apiClient.fetchTo(
            `${this.baseUri}/item_inventories${page ? `?page=${page}` : ``}`);
    }

    public async getById(id: number): Promise<ItemInventory | undefined> {
        return await this.apiClient.fetchTo(
            `${this.baseUri}/item_inventories/${id}`);
    }

    public async getByItemName(itemName: string, page?: number): Promise<PagedData<ItemInventory> | undefined> {
        return await this.apiClient.fetchTo(
            `${this.baseUri}/item_inventories?itemName=${encodeURIComponent(itemName)}${page ? `&page=${page}` : ``}`);
    }

    public async post(itemInventory: ItemInventory): Promise<ItemInventory> {
        return await this.apiClient.fetchTo(
            `${this.baseUri}/item_inventories`, {
                method: `POST`,
                body: JSON.stringify(itemInventory),
            });
    }

    public async patchById(id: number, itemInventory: ItemInventory): Promise<ItemInventory> {
        return await this.apiClient.fetchTo(
            `${this.baseUri}/item_inventories/${id}`, {
                method: `PATCH`,
                headers: {
                    "Content-Type": "application/merge-patch+json",
                },
                body: JSON.stringify(itemInventory),
            });
    }
}
