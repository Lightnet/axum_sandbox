import { createMemo, createSignal, onMount } from "solid-js"


export default function PageTask(){

  const [content, setContent] = createSignal('testss')

  const [tasks, setTasks] = createSignal([])
  const [editID, setEditID] = createSignal('')
  const [editContent, setEditContent] = createSignal('')

  async function getTask(){
    try{
      let resp = await fetch('/api/task');
      let data = await resp.json();
      console.log(data)
      if(data){
        setTasks(data)
      }

    }catch(e){
      console.log("Error:",e.message);
    }
  }

  async function onAddTask(){
    console.log("add")
    try{
      let resp = await fetch('/api/task',{
        method:'POST',
        headers: {
          //'Accept': 'application/json',
          'Content-Type': 'application/json'
        },
        body:JSON.stringify({
          title:"none",
          text:content()
        })
      });
      let data = await resp.json();
      console.log(data)
      if(data){
        //setTasks(data)
        if(data.api=='created'){
          setTasks(state=>[...state,{
            id:data.id,
            text:content()
          }])
        }
      }

    }catch(e){
      console.log("Error:",e.message);
    }
  }

  async function updateTask(id){
    try{
      let resp = await fetch('/api/task',{
        method:'PUT',
        headers:{
          'Content-Type': 'application/json'
        },
        body:JSON.stringify({
          id:editID(),
          text:editContent()
        })
      });
      let data = await resp.json();
      console.log(data)
      if(data){
        //setTasks(data)
        if(data.api && data.api == 'updated'){
          setTasks(state=>state.map(item=>{
            //console.log(item)
            if(item.id == editID()){
              return { ...item, text: editContent()}
            }else{
              return item;
            }
          }))
        }
      }

    }catch(e){
      console.log("Error:",e.message);
    }

    setEditContent('')
    setEditID('')
  }

  function editTask(id){
    let tasklist = tasks()
    let taskdata = tasklist.find(item=>item.id == id);
    setEditContent(taskdata.text)
    setEditID(id)
  }

  async function deleteTask(id){
    try{
      let resp = await fetch(`/api/task/${id}`,{
        method:'DELETE'
      });
      let data = await resp.json();
      console.log(data)
      if(data.api == 'delete'){
        setTasks(state=>state.filter(item=>item.id != id))
      }
    }catch(e){
      console.log("Error:",e.message);
    }
  }

  const taskList = createMemo(()=>{
    let list = tasks()
    if(list){
      list = list.map(item=>{
        return <>
        <li id={item.id}>
           {editID() == item.id ? <>
            <input value={editContent()} onInput={(e)=>{setEditContent(e.target.value)}} />
            <button onClick={()=>updateTask(item.id)}>Update</button>
           </>:<>
            <label>{item.text} </label>
            <button onClick={()=>editTask(item.id)}>Edit</button>
           </>}
           
           <button onClick={()=>deleteTask(item.id)}>Delete</button>
        </li>
        </>
      })
      return list;
    }
    return []
  })

  onMount(()=>{
    getTask()
  })

  return <>
    <div>
      <input value={content()} onInput={(e)=>setContent(e.target.value)} /><button onClick={onAddTask}>Add Task</button>
      <div>
        <ul>
          {taskList}
        </ul>
      </div>
    </div>
  </>
}