crate=`cargo metadata --format-version=1 | jq -r '.packages[] | select(.name == "'$1'") | select(.manifest_path ) | .manifest_path' | sort | head -n 1`
workspace=$(echo $crate | sed 's#crates.*#Cargo.toml#')
echo $crate 
echo `dirname $workspace`
exit
cd `dirname $workspace`
git apply $2
git diff | cat
#git checkout .
cargo clean -p $1
#echo $crate
