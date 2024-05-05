import json
from typing import Dict, Set, Tuple

import requests

list_of_archs = [
    # "srpm",
    # "noarch",
    "x86_64",
    "i586",
    "aarch64",
    "ppc64le",
    # "x86_64-i586",
]

list_of_repos = ["sisyphus", "p10", "p9"]

api_url = "https://rdb.altlinux.org/api/export/branch_binary_packages/"


def get_packages_set(repo: str) -> Dict[str, Set[str]]:
    packages = {}

    for arch in list_of_archs:
        full_url = api_url + repo + f"?arch={arch}"

        response = requests.get(full_url)
        if response.status_code == 200:
            data = [package["name"] for package in response.json()["packages"]]
            packages[arch] = data
            print(f"[+] Success: {repo} {arch}")
        else:
            print(f"[-] Error: {repo} {arch}")
    print()

    with open(f"{repo}", "w") as file:
        json.dump(packages, file, indent=4)

    return packages


def get_repo_difference(
    packages_1: Set[str], packages_2: Set[str]
) -> Tuple[Set[str], Set[str], Set[str]]:
    return (
        packages_1.difference(packages_2),
        packages_2.difference(packages_1),
        packages_1.intersection(packages_2),
    )


def main() -> None:
    all_packages = []

    for repo in list_of_repos:
        all_packages.append(get_packages_set(repo))

    res = get_repo_difference(
        set(all_packages[0]["x86_64"]), set(all_packages[1]["x86_64"])
    )

    print(len(res[0]), len(res[1]), len(res[2]))


if __name__ == "__main__":
    main()
