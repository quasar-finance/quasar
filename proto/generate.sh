#cd proto

PWD=$(pwd)
echo "pwd is $PWD"
cd proto
PWD=$(pwd)
echo "pwd is $PWD"
# buf generate
buf generate --template buf.gen.gogo.yaml
cd ..

echo "Copying ..."
PWD=$(pwd)
echo "$PWD"
cp -r github.com/quasar-finance/quasar/* ./
# rm -rf github.com

