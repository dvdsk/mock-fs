source $(dirname "$0")/../deploy.sh


# check which run this will be
run_numb=$((ls *.last_run 2>/dev/null || echo "-1.run_numb") \
	| sort \
	| head -n 1 \
	| cut -d " " -f 2 \
	| cut -d "." -f 1)
run_numb=$(($run_numb + 1))
rm *.last_run >& /dev/null
touch $run_numb.last_run

# deploy cluster
numb_nodes=3
bin="meta-server"
args="
	--client-port 50975 \
	--cluster-size $numb_nodes \
	--control-port 50972 \
	--tracing-endpoint fs1.cm.cluster \
	--run-numb $run_numb"

res=$(deploy $numb_nodes $bin $args)
echo $res
resv_numb=$(echo $res | cut -d " " -f 1)
nodes=$(echo $res | cut -d " " -f 2-)

sleep 10
echo nodes captured: $nodes
echo resv numb: $resv_numb
# deploy 1 test_mkdir $nodes

cleanup $resv_numb
