4验证人node-key(使用subkey生成)

12D3KooWMwqgHC8bsZ7nixSagXUhZ6wG4qURLXCm4Q9YzzGr6Cee
758426877c0b54bf656b5ec804b1a514da6573341948c2f4e2a5251ac17af4c2

12D3KooWLfM4y2FHZxMrNmyD6Xon4tC55T1qCEtiB2mJq8WcpfYH
7ed8339f7523ac2709deec4753ec4a7cb1f7a09069ed58659c85c82e6641c9c5

12D3KooWG6n3bXcogzzmvstDMRBaYwSadHHnVCMzkF6NSt2pbAWm
7311e3909937398c074a20a82470f9b3658dfe49cb2e9d2a7acfa6ca8ed3dc78

12D3KooWBpV8LsrpxdCLeEfWwVDaEtVijdB8hMtJ6mC1TaXUQFAF
ea8a1d99f96b7680419eaccaa40c73d61ac1c19801392f064255d49d93bc02e7

2验证人节点运行指令

./target/release/cycan   --base-path /tmp/alice   --chain=local   --alice   --node-key 0000000000000000000000000000000000000000000000000000000000000001   --validator --rpc-cors=all --no-mdns

./target/release/cycan   --base-path /tmp/bob   --chain=local   --bob   --port 30334 --ws-port 9946 --rpc-port 9934 --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp   --validator --rpc-cors=all --no-mdns

4验证人节点运行指令

./target/release/cycan   --base-path /tmp/alice   --chain=beta   --bootnodes "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWMwqgHC8bsZ7nixSagXUhZ6wG4qURLXCm4Q9YzzGr6Cee" "/ip4/127.0.0.1/tcp/30334/p2p/12D3KooWLfM4y2FHZxMrNmyD6Xon4tC55T1qCEtiB2mJq8WcpfYH" "/ip4/127.0.0.1/tcp/30335/p2p/12D3KooWG6n3bXcogzzmvstDMRBaYwSadHHnVCMzkF6NSt2pbAWm" "/ip4/127.0.0.1/tcp/30336/p2p/12D3KooWBpV8LsrpxdCLeEfWwVDaEtVijdB8hMtJ6mC1TaXUQFAF" --node-key 758426877c0b54bf656b5ec804b1a514da6573341948c2f4e2a5251ac17af4c2   --validator --rpc-cors=all --no-mdns --keystore-path ./keystore/node1

./target/release/cycan   --base-path /tmp/bob   --chain=beta   --port 30334 --ws-port 9946 --rpc-port 9934 --bootnodes "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWMwqgHC8bsZ7nixSagXUhZ6wG4qURLXCm4Q9YzzGr6Cee" "/ip4/127.0.0.1/tcp/30334/p2p/12D3KooWLfM4y2FHZxMrNmyD6Xon4tC55T1qCEtiB2mJq8WcpfYH" "/ip4/127.0.0.1/tcp/30335/p2p/12D3KooWG6n3bXcogzzmvstDMRBaYwSadHHnVCMzkF6NSt2pbAWm" "/ip4/127.0.0.1/tcp/30336/p2p/12D3KooWBpV8LsrpxdCLeEfWwVDaEtVijdB8hMtJ6mC1TaXUQFAF" --node-key 7ed8339f7523ac2709deec4753ec4a7cb1f7a09069ed58659c85c82e6641c9c5  --validator --rpc-cors=all --no-mdns --keystore-path ./keystore/node2 --execution NativeElseWasm

./target/release/cycan   --base-path /tmp/charlie   --chain=beta   --port 30335 --ws-port 9947 --rpc-port 9935 --bootnodes "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWMwqgHC8bsZ7nixSagXUhZ6wG4qURLXCm4Q9YzzGr6Cee" "/ip4/127.0.0.1/tcp/30334/p2p/12D3KooWLfM4y2FHZxMrNmyD6Xon4tC55T1qCEtiB2mJq8WcpfYH" "/ip4/127.0.0.1/tcp/30335/p2p/12D3KooWG6n3bXcogzzmvstDMRBaYwSadHHnVCMzkF6NSt2pbAWm" "/ip4/127.0.0.1/tcp/30336/p2p/12D3KooWBpV8LsrpxdCLeEfWwVDaEtVijdB8hMtJ6mC1TaXUQFAF" --node-key 7311e3909937398c074a20a82470f9b3658dfe49cb2e9d2a7acfa6ca8ed3dc78   --validator --rpc-cors=all --no-mdns --keystore-path ./keystore/node3

./target/release/cycan   --base-path /tmp/dave   --chain=beta   --port 30336 --ws-port 9948 --rpc-port 9936 --bootnodes "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWMwqgHC8bsZ7nixSagXUhZ6wG4qURLXCm4Q9YzzGr6Cee" "/ip4/127.0.0.1/tcp/30334/p2p/12D3KooWLfM4y2FHZxMrNmyD6Xon4tC55T1qCEtiB2mJq8WcpfYH" "/ip4/127.0.0.1/tcp/30335/p2p/12D3KooWG6n3bXcogzzmvstDMRBaYwSadHHnVCMzkF6NSt2pbAWm" "/ip4/127.0.0.1/tcp/30336/p2p/12D3KooWBpV8LsrpxdCLeEfWwVDaEtVijdB8hMtJ6mC1TaXUQFAF" --node-key ea8a1d99f96b7680419eaccaa40c73d61ac1c19801392f064255d49d93bc02e7   --validator --rpc-cors=all --no-mdns --keystore-path ./keystore/node4
