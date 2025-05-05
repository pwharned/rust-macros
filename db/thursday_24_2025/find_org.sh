CODE=$(curl --negotiate -u : --location --silent "https://bluepages.ibm.com/BpHttpApisv3/slaphapi?ibmperson/(mail=$1).search/byjson" | jq -r '.search.entry[0].attribute[] | select(.name == "hrorganizationcode") | .value[0]' | xargs -I{} curl --negotiate -u : --location --silent "https://bluepages.ibm.com/BpHttpApisv3/slaphapi?ibmorganization/hrorganizationcode={}/byldif?*" | grep "^hrOrganizationDisplay:" | cut -d: -f2- | sed 's/^ //')

echo $1\;$CODE;


