'use client';

import {useEffect, useMemo, useState} from "react";
import Content from "@/components/Content";
import Table, {PagedData} from "@/components/Table";
import Modal, {ComponentType} from "@/components/Modal";
import UserProfilesApiClient, {UserProfile} from "@/utils/UserProfilesApiClient";
import {useSession} from "next-auth/react";
import {User} from "next-auth";

const UserProfiles = () => {
    const apiBaseUri = process.env.NEXT_PUBLIC_API_BASE_URI;

    const { data } = useSession();
    const user: User | undefined = data?.user;
    const accessToken = data?.accessToken;

    const [userProfiles, setUserProfiles] = useState<PagedData<UserProfile> | null>();
    const [editModalShown, setEditModalShown] = useState(false);
    const [editModalData, setEditModalData] = useState<any | null | undefined>(null);
    const [addModalShown, setAddModalShown] = useState(false);
    const [addModalData, setAddModalData] = useState<any | null | undefined>(null);

    const userProfilesApiClient = useMemo(
        () => new UserProfilesApiClient(accessToken, apiBaseUri),
        [accessToken, apiBaseUri]);

    useEffect(() => {
        const init = async () => {
            try {
                setUserProfiles(await userProfilesApiClient.get());
            } catch (e) {
                console.error(e);
            }
        };

        try {
            void init();
        } catch (e) {
            console.error(e);
        }
    }, [accessToken]);

    return (
        <Content title={`User Profiles`}>
            <Table<UserProfile>
                data={userProfiles}
                searchLabel={`Username`}
                onSearch={async (v) => {
                    try {
                        setUserProfiles(await userProfilesApiClient.getByUsername(v));
                    } catch (e) {
                        console.error(e);
                    }
                }}
                onAdd={() => {
                    setAddModalData({
                        id: ``,
                        username: ``,
                        familyName: ``,
                        givenName: ``,
                        age: ``,
                        gender: 0,
                        profile: ``,
                    });

                    setAddModalShown(true);
                }}
                onEdit={(data) => {
                    setEditModalData(data);

                    setEditModalShown(true);
                }}
                onClickPage={async (keyword, page) => {
                    try {
                        setUserProfiles(await userProfilesApiClient.getByUsername(keyword, page));
                    } catch (e) {
                        console.error(e);
                    }
                }}
            />
            <Modal open={editModalShown}
                   title={`Edit User Profile`}
                   data={editModalData}
                   dataConfig={[{
                       name: `id`,
                       type: ComponentType.Label,
                   }, {
                       name: `username`,
                       type: ComponentType.Label,
                   }, {
                       name: `profile`,
                       type: ComponentType.TextArea,
                       colspan: 2,
                   }, {
                       name: `gender`,
                       type: ComponentType.Select,
                       options: [{
                           label: `Male`,
                           value: 1,
                       }, {
                           label: `Female`,
                           value: 2,
                       },]
                   }]}
                   save={{
                       onClick: async (data) => {
                           const existedUserProfile = (userProfiles?.content ?? []).find(d => data.id === d.id);
                           const userId = existedUserProfile?.id;

                           if (!userId) {
                               return;
                           }

                           try {
                               const patchUserProfile: UserProfile = {
                                   id: userId,
                                   family_name: data.family_name ?? existedUserProfile.family_name,
                                   given_name: data.given_name ?? existedUserProfile.given_name,
                                   age: data.age ?? existedUserProfile.age,
                                   gender: data.gender ?? existedUserProfile.gender,
                                   profile: data.profile ?? existedUserProfile.profile,
                               } as UserProfile

                               await userProfilesApiClient.patchById(
                                   userId, patchUserProfile);

                               setUserProfiles(await userProfilesApiClient.get());
                               setEditModalShown(false);
                           } catch (e) {
                               console.error(e);
                           }
                       }
                   }}
                   cancel={{
                       onClick: () => setEditModalShown(false)
                   }}
            />
            <Modal open={addModalShown}
                   title={`Add User Profile`}
                   data={addModalData}
                   dataConfig={[{
                       name: `id`,
                       type: ComponentType.Label,
                   }, {
                       name: `profile`,
                       type: ComponentType.TextArea,
                       colspan: 2,
                   }, {
                       name: `gender`,
                       type: ComponentType.Select,
                       options: [{
                           label: `Male`,
                           value: 1,
                       }, {
                           label: `Female`,
                           value: 2,
                       },]
                   }]}
                   save={{
                       onClick: async (data) => {
                           if (!data.id) {
                               return;
                           }

                           try {
                               const newData = await userProfilesApiClient.post({
                                   username: data.username,
                                   family_name: data.family_name,
                                   given_name: data.given_name,
                                   age: data.age,
                                   gender: data.gender,
                                   profile: data.profile,
                               });
                               setUserProfiles({
                                   ...userProfiles,
                                   content: [
                                       ...(userProfiles?.content ?? []),
                                       newData,
                                   ]
                               } as PagedData<UserProfile>);

                               setAddModalShown(false);
                           } catch (e) {
                               console.error(e);
                           }
                       }
                   }}
                   cancel={{
                       onClick: () => setAddModalShown(false)
                   }}
            />
        </Content>
    );
}

export default UserProfiles;
