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
cp -r github.com/quasarlabs/quasarnode/* ./
# rm -rf github.com

