'use client';

import ApiClient from "@/utils/ApiClient";
import {PagedData} from "@/components/Table";

export type UserProfile = {
    id?: string;
    username: string;
    family_name: string;
    given_name: string;
    age: number;
    gender: number;
    profile: string;
};

export default class usersApiClient {
    private readonly apiClient: ApiClient
    private readonly baseUri: string

    constructor(accessToken: string | undefined, baseUri: string | undefined) {
        this.apiClient = new ApiClient(accessToken);
        this.baseUri = baseUri ?? ``;
    }

    public async get(page?: number): Promise<PagedData<UserProfile> | undefined> {
        return await this.apiClient.fetchTo(
            `${this.baseUri}/users${page ? `?page=${page}` : ``}`);
    }

    public async getByUsername(username: string, page?: number): Promise<PagedData<UserProfile> | undefined> {
        return await this.apiClient.fetchTo(
            `${this.baseUri}/users?username=${encodeURIComponent(username)}${page ? `&page=${page}` : ``}`);
    }

    public async post(userProfile: UserProfile): Promise<UserProfile> {
        return await this.apiClient.fetchTo(
            `${this.baseUri}/users`, {
                method: `POST`,
                body: JSON.stringify(userProfile),
            });
    }

    public async patchById(id: string, userProfile: UserProfile): Promise<UserProfile> {
        return await this.apiClient.fetchTo(
            `${this.baseUri}/users/${id}`, {
                method: `PATCH`,
                headers: {
                    "Content-Type": "application/merge-patch+json",
                },
                body: JSON.stringify(userProfile),
            });
    }
}
