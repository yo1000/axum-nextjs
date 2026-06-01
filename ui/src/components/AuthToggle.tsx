"use client"

import {signIn, signOut, useSession} from "next-auth/react";
import {User} from "next-auth";
import React, {Suspense, useEffect, useState} from "react";
import {DisclosureButton} from "@headlessui/react";
import {usePathname, useSearchParams} from "next/navigation";

export default function AuthToggle({ shortWidth = false, }: { shortWidth: boolean, }) {
    return (
        <Suspense fallback={(
            shortWidth ? <SignInShortWidth/> : <SignInFullWidth/>
        )}>
            <AuthContent shortWidth={ shortWidth }/>
        </Suspense>
    );
}

function AuthContent({ shortWidth = false, }: { shortWidth: boolean, }) {
    // 'loading' | 'authenticated' | 'unauthenticated'
    const { update, data, status } = useSession();
    // const [user, setUser] = useState(data?.user);

    const pathname = usePathname();
    const searchParams = useSearchParams();

    useEffect(() => {
        (async () => {
            // TODO
            void await update();
            // const session = await update();
            // setUser(data?.user);
        })();
    }, [pathname, searchParams]);

    if (status === "authenticated") {
        return shortWidth ? <SignOutShortWidth user={data?.user}/> : <SignOutFullWidth user={data?.user}/>;
    } else {
        return shortWidth ? <SignInShortWidth/> : <SignInFullWidth/>;
    }
}

function SignInFullWidth() {
  return (
    <button
      className={`text-neutral-300 hover:bg-neutral-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium ring-1 ring-inset ring-neutral-300`}
      // TODO: Provider Id change to be Variable.
      onClick={() => {void signIn("keycloak", { redirectTo: "/"})}}>
      Sign in
    </button>
  );
}

function SignInShortWidth() {
  return (
    <DisclosureButton
      as="span"
      className={"block rounded-md px-3 py-2 text-base font-medium text-neutral-400 hover:bg-neutral-700 hover:text-white"}
      onClick={void signIn("keycloak", { redirectTo: "/" })}>
      Sign in
    </DisclosureButton>
  );
}

function SignOutFullWidth({ user, }: { user?: User }) {
  return (
    <>
      <span className={`text-neutral-300 rounded-md px-3 py-2 text-sm font-medium`}>
        {user?.name}
      </span>
      <button
        className={`text-neutral-300 hover:bg-neutral-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium ring-1 ring-inset ring-neutral-300`}
        onClick={() => signOut({redirectTo: "/"})}>
        Sign out
      </button>
    </>
  );
}

function SignOutShortWidth({ user, }: { user?: User }) {
  return (
    <>
      <DisclosureButton as="span" className="block rounded-md px-3 py-2 text-base font-medium text-neutral-400">
        {user?.name}
      </DisclosureButton>
      <DisclosureButton
        as="span"
        className="block rounded-md px-3 py-2 text-base font-medium text-neutral-400 hover:bg-neutral-700 hover:text-white"
        onClick={() => signOut({redirectTo: "/"})}>
        Sign out
      </DisclosureButton>
    </>
  );
}
