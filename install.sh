echo "cargo bundle --release"
cargo bundle --release

APP_NAME="Javdesk"
APPLICATIONS_PATH="/Applications"
APP_PATH="${APPLICATIONS_PATH}/${APP_NAME}.app"

if [[ -e ${APP_PATH} ]]
then
    echo "file ${APP_PATH} exists"
    rm -r ${APP_PATH}
    echo "file ${APP_PATH} removed"
fi

echo "move ${APP_NAME} to Applications"
mv ./target/release/bundle/osx/${APP_NAME}.app ${APPLICATIONS_PATH}
