PACKAGE_NAME=(`awk -F ' = ' '$1 ~ /name/ { gsub(/[\"]/, "", $2); printf("%s",$2) }' ./$1/Cargo.toml`)
echo $PACKAGE_NAME