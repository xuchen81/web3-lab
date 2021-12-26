/**
 * @type import('hardhat/config').HardhatUserConfig
 */

require("@nomiclabs/hardhat-waffle");

const INFURA_URL = "https://rinkeby.infura.io/v3/<id>";
const PRIVATE_KEY = "<private-key>";

module.exports = {
  solidity: "0.8.1",
  networks: {
    rinkeby: {
      url: INFURA_URL,
      accounts: [`0x${PRIVATE_KEY}`],
    },
  },
};
