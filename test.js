var script = Script.setup("KillAura", "Adds another killaura", "1.0.0", ["Rythem"])
var module = ModuleHandler.create(script, "Kill Aura 1", "Allows you to have 2 killauras", 0, Category.COMBAT)

var cps = Setting.createSlider("CPS", "The rate of clicks per second", 9, 1, 20, 1)
var range = Setting.createSlider("Range", "The distance to attack", 3, 3, 4, 0.1)

var players = Setting.createSwitch("Players", "Should it attack players", true)
var animals = Setting.createSwitch("Animals", "Should it attack animals", true)
var mobs = Setting.createSwitch("Mobs", "Should it attack mobs", true)

var attack = Setting.createSwitch("Attack", "Should it attack", true)

var clientswing = Setting.createSwitch("Client Swing", "Should it swing on the client", true)
var serverswing = Setting.createSwitch("Server Swing", "Should it swing on the server", true)

var rotate = Setting.createSwitch("Rotate", "Should it rotate", true)

var movementfix = Setting.createSwitch("Movement Fix", "Should it fix movement", true)
var silentmovementfix = Setting.createSwitch("Silent Movement Fix", "Should it make the movement fix silent", true)

Setting.register(module, cps, range, players, animals, mobs, attack, clientswing, serverswing, rotate, movementfix, silentmovementfix)

var prevTime = 0
var delay = 0
var yaw = -999
var pitch = -999
var target = null

module.onEnable(function() {
    target = null
    yaw = -999
    pitch = -999
})

module.onMotion(function(event) {
    if(target != null && rotate.getState() && (yaw != -999 || pitch != -999)) {
        event.setRotationYaw(yaw)
        event.setRotationPitch(pitch)
    }
})

module.onMoveFlying(function(event) {
    var targets = World.getEntities(range.getValue(), true, players.getState(), animals.getState(), mobs.getState(), false, false, false, false, false)
    targets = World.sortByDistance(targets, Player.getPosition(), true) 
    
    if(targets.size() > 0) {
        target = Entity.asLivingEntity(targets.get(0))
        
        if(target != null) {
            var rotations = Rotation.find(target)
            rotations = Rotation.applyGCD(rotations, Player.getRotations())
            
            if(rotate.getState()) {
                yaw = rotations[0]
                pitch = rotations[1]
            }
            
            var time = Client.getTime()
            if(attack.getState() && (time - prevTime) > delay) {
                Player.swing(clientswing.getState(), serverswing.getState())
                Packet.send(Packet.C02PacketUseEntity(target, C02Action.ATTACK))
                
                prevTime = time
                delay = (1000.0 / cps.getValue())
            }
        }
    } else {
        target = null
        yaw = -999
        pitch = -999
    }
    
    if(rotate.getState() && target != null && (yaw != -999 || pitch != -999) && movementfix.getState())
        event.setRotationYaw(yaw)
})

module.onJump(function(event) {
  if(rotate.getState() && target != null && (yaw != -999 || pitch != -999) && movementfix.getState())
    event.setRotationYaw(yaw)
})

module.onMovementInput(function(event) {
  if(rotate.getState() && target != null && (yaw != -999 || pitch != -999) && movementfix.getState() && silentmovementfix.getState()) {
    event.setRotationYaw(yaw)
    event.cancel()
  }
})

script.onLoad(function() {
    ModuleHandler.register(module)
})

script.onUnload(function() {
    ModuleHandler.unregister(module)
})
